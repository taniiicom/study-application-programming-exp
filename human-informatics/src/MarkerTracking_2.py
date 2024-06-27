import numpy as np
import cv2
import math

CONTOUR_AREA_THRESHOLD = 400

def prepare_camera(camera_id, fps, width, height):
    cap = cv2.VideoCapture(camera_id)
    if not cap.isOpened():
        raise ValueError(f"Camera {camera_id} could not be opened.")
    cap.set(cv2.CAP_PROP_FPS, fps)
    cap.set(cv2.CAP_PROP_FRAME_WIDTH, width)
    cap.set(cv2.CAP_PROP_FRAME_HEIGHT, height)
    
    # ここで落ちる場合は、指定した値にカメラが対応していない場合がある
    assert cap.get(cv2.CAP_PROP_FRAME_WIDTH) == width, "Width setting failed."
    assert cap.get(cv2.CAP_PROP_FRAME_HEIGHT) == height, "Height setting failed."

    return cap

def line_intersection(A1, A2, B1, B2):
    """ returns a tuple (x, y) or None if there is no intersection """
    d = (B2[1] - B1[1]) * (A2[0] - A1[0]) - (B2[0] - B1[0]) * (A2[1] - A1[1])
    if d:
        uA = ((B2[0] - B1[0]) * (A1[1] - B1[1]) - (B2[1] - B1[1]) * (A1[0] - B1[0])) / d
        uB = ((A2[0] - A1[0]) * (A1[1] - B1[1]) - (A2[1] - A1[1]) * (A1[0] - B1[0])) / d
    else:
        return
    if not(0 <= uA <= 1 and 0 <= uB <= 1):
        return
    x = int(A1[0] + uA * (A2[0] - A1[0]))
    y = int(A1[1] + uA * (A2[1] - A1[1]))
 
    return x, y

def calc_distance(p1, p2):
    distance = math.sqrt(((p1[0]-p2[0])**2)+((p1[1]-p2[1])**2))
    return distance

def get_line_endpoints(lines):
    """ Hough変換で得られた直線の端点を計算する """
    points1 = []
    points2 = []
    if lines is not None:
        for line in lines:
            rho, theta = line[0]
            a = math.cos(theta)
            b = math.sin(theta)
            x0 = a * rho
            y0 = b * rho
            points1.append((int(x0 + 2000 * (-b)), int(y0 + 2000 * a)))
            points2.append((int(x0 - 2000 * (-b)), int(y0 - 2000 * a)))
    return points1, points2

def detect_intersects(lines, frame):
    """ 交点を検出 """
    intersects_uv = [[0, 0] for n in range(4)] # Four intersection points on monitor
    counts_intersect_grp = [0 for m in range(4)] # number of nearest points of each intersection point
    points1, points2 = get_line_endpoints(lines)
    if lines is not None:
        for i in range(0, len(lines)):
            cv2.line(frame, points1[i], points2[i], (0,0,255), 1, cv2.LINE_AA)
            for j in range(0, i):
                # 2直線i, jのペアを考える
                # なす角の計算
                theta1, theta2 = lines[i][0][1], lines[j][0][1]
                angle = abs(theta1 - theta2)
                if angle > np.pi:
                    angle = 2 * np.pi - angle
                angle_degree = math.degrees(angle)
                if angle_degree < 60 or 120 < math.degrees(angle): # なす角が90度でないペアは無視
                    continue

                pt = line_intersection(points1[j], points2[j], points1[i], points2[i])
                if pt is None:
                    continue
                cv2.circle(frame, pt, 2, (255, 255, 255), -1)
                for idx in range(4):
                    distance = calc_distance(intersects_uv[idx], pt)
                    if distance < 10 or counts_intersect_grp[idx] == 0: # the same group
                        # update intersection coordinates by averaging
                        intersects_uv[idx][0] = \
                            (intersects_uv[idx][0] * counts_intersect_grp[idx] + pt[0]) / (counts_intersect_grp[idx] + 1)
                        intersects_uv[idx][1] = \
                            (intersects_uv[idx][1] * counts_intersect_grp[idx] + pt[1]) / (counts_intersect_grp[idx] + 1)
                        counts_intersect_grp[idx] += 1
                        break
    return intersects_uv, counts_intersect_grp

def match_points(points_prev, points_new):
    """ 前フレームで検出した点と今回の点のマッチング """
    ret = list()
    remaining_idxs = [0, 1, 2, 3]
    for idx_prev in range(4):
        _distances = [calc_distance(points_prev[idx_prev], points_new[idx]) for idx in remaining_idxs]
        _nearest_idx = remaining_idxs[np.argmin(_distances)]
        ret.append(points_new[_nearest_idx])
        remaining_idxs.remove(_nearest_idx)
    return ret


def main_loop(cap, thresholds, params):
    IMAGE_NAMES = ['Original image', 'Gray image', 'Binary image', 'Edge image']
    image_mode = 0
    CAMERA_WIDTH, CAMERA_HEIGHT = cap.get(cv2.CAP_PROP_FRAME_WIDTH), cap.get(cv2.CAP_PROP_FRAME_HEIGHT)

    recording = False
    init_points = True
    intersects_uv_prev = None # Four edge points on monitor of previous loop

    while True:
        key = cv2.waitKey(1) & 0xFF
        if key == ord('q'):
            break
        elif key == ord('t'):
            cv2.destroyWindow(IMAGE_NAMES[image_mode])
            image_mode = (image_mode + 1) % len(IMAGE_NAMES)
        elif key == ord('r'):
            init_points = True
        elif key == ord('s'):
            if not recording:
                print('start recording')
                # TODO: バッファーの初期化
                recording = True
            else:
                print('saving finished')
                # TODO: バッファーの内容を保存
                recording = False
        # TODO (他のキー操作)
        
        ret, frame = cap.read()
        gray_frame = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
        
        #グレースケールを2値化画像に変換 
        _, im_th = cv2.threshold(gray_frame, thresholds['binary'], 255, cv2.THRESH_BINARY)
        #th, im_th = cv2.threshold(gray_frame, 0, 255, cv2.THRESH_OTSU)
        binary_frame = cv2.bitwise_not(im_th)
        contours, hierarchy = cv2.findContours(binary_frame, cv2.RETR_LIST, cv2.CHAIN_APPROX_SIMPLE)
        
        mask = np.zeros(binary_frame.shape, np.uint8)
        if len(contours) > 0:
            largest_contour = max(contours, key=cv2.contourArea)
            #入力画像のモーメント
            mu = cv2.moments(largest_contour)
            #モーメントからu,v座標を計算
            if mu["m00"] > 0 and cv2.contourArea(largest_contour) > CONTOUR_AREA_THRESHOLD:
                cv2.drawContours(mask, [largest_contour], 0, 255, -1)
                    
        edge_img = cv2.Canny(mask, thresholds['canny_low'], thresholds["canny_high"])
        # Standard Hough Line Transform
        lines = cv2.HoughLines(edge_img, 1, np.pi / 180, thresholds['hough'], None, 0, 0)

        intersects_uv, counts_intersect_grp = detect_intersects(lines, frame)

        for idx in range(4):
            if counts_intersect_grp[idx] != 0:
                cv2.circle(frame, list(map(int, intersects_uv[idx])), 6, (255, 0, 0), 1)
                
        if all(counts_intersect_grp): # 交点を4点検出したら
            if init_points == True: #点の順番のリセット
                init_points = False
                frame_edges = [[0, CAMERA_HEIGHT], [0, 0], [CAMERA_WIDTH, 0], [CAMERA_WIDTH, CAMERA_HEIGHT]]
                # Reset intersection ids
                intersects_uv_prev = match_points(frame_edges, intersects_uv)

            # Match intersection ids with the previous ones
            intersects_uv = match_points(intersects_uv_prev, intersects_uv)

            ############　TODO　(x, y, z, roll, pitch, yawの計算)###########
            fx, fy, d = params['fx'], params['fy'], params['d']
            for i, (u, v) in enumerate(intersects_uv):
                text = f'p{i}: ({int(u)}, {int(v)})'
                cv2.putText(frame, text, (int(u) + 20, int(v) + 20),
                    cv2.FONT_HERSHEY_PLAIN, 1.0,
                    (255, 255, 255), 1, cv2.LINE_AA)
                ######U, Vを計算#########
            
            ### p0, p1, p2, p3を計算###

            ### Roll, Pitch, Yawを計算 ###
            

        #TODO (bufferにデータを保存)
        #if recording == True:
            # ToDo: バッファに保存


        if image_mode == 0:
            show_img = frame
        elif image_mode == 1:
            show_img = gray_frame
        elif image_mode == 2:
            show_img = binary_frame
        elif image_mode == 3:
            show_img = edge_img
        
        cv2.imshow(IMAGE_NAMES[image_mode], show_img)
        intersects_uv_prev = intersects_uv
    cv2.destroyAllWindows()
     
if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser(description="Marker detection using a camera.")
    parser.add_argument('--camera_id', type=int, default=0, help='Camera index')
    parser.add_argument('--fps', type=int, default=30, help='Frames per second')
    parser.add_argument('--camera_width', type=int, default=640, help='Frame width')
    parser.add_argument('--camera_height', type=int, default=480, help='Frame height')
    parser.add_argument('--binary_threshold', type=int, default=50, help='Thresohld for binarization')
    parser.add_argument('--canny_low', type=int, default=100, help='Thresohld for Canny Edge Detection (low)')
    parser.add_argument('--canny_high', type=int, default=300, help='Thresohld for Canny Edge Detection (high)')
    parser.add_argument('--hough_threshold', type=int, default=50, help='Thresohld for Hough Line Transform')

    parser.add_argument('--fx', type=float, help='fx')
    parser.add_argument('--fy', type=float, help='fy')
    parser.add_argument('--d', type=float, help='d')
    args = parser.parse_args()

    # 閾値 0-255
    thresholds = {
        'binary': args.binary_threshold,
        'canny_low': args.canny_low,
        'canny_high': args.canny_high,
        'hough': args.hough_threshold
    }
    params = {
        'fx': args.fx,
        'fy': args.fy,
        'd': args.d,
    }

    cap = prepare_camera(camera_id=args.camera_id, fps=args.fps, width=args.camera_width, height=args.camera_height)
    main_loop(cap, thresholds, params)
    cap.release()
