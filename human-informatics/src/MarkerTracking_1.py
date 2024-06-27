import csv
import cv2
import math
import numpy as np
import time

CONTOUR_AREA_THRESHOLD = 400

def prepare_camera(camera_id, fps, width, height):
    cap = cv2.VideoCapture(camera_id)
    if not cap.isOpened():
        raise ValueError(f"Camera {camera_id} could not be opened.")
    cap.set(cv2.CAP_PROP_FPS, fps)
    cap.set(cv2.CAP_PROP_FRAME_WIDTH, width)
    cap.set(cv2.CAP_PROP_FRAME_HEIGHT, height)
    
    assert cap.get(cv2.CAP_PROP_FRAME_WIDTH) == width, "Width setting failed."
    assert cap.get(cv2.CAP_PROP_FRAME_HEIGHT) == height, "Height setting failed."

    return cap

class Buffer():
    def __init__(self, keys=['u', 'v']):
        self.data = list()
        self.keys = keys
        self.start_time = time.time()

    def append(self, **kwargs):
        # 時刻も保存
        t = 1000 * (time.time() - self.start_time)
        item = [t] + [kwargs[key] for key in self.keys]
        self.data.append(item)

    def dump(self, csv_path='log.csv'):
        header = ['t'] + self.keys
        with open(csv_path, 'w') as f:
            writer = csv.writer(f)
            writer.writerow(header)
            writer.writerows(self.data)


def detect_marker(contours):
    # 画像モーメントをもとに、マーカーの座標を計算
    if len(contours) > 0:
        largest_contour = max(contours, key=cv2.contourArea)
        # 入力画像のモーメント
        mu = cv2.moments(largest_contour)
        # 面積
        s = cv2.contourArea(largest_contour)
        if mu["m00"] > 0 and s > CONTOUR_AREA_THRESHOLD:
            # モーメントからu,v座標を計算
            u, v = int(mu["m10"] / mu["m00"]) , int(mu["m01"] / mu["m00"])
            r_dot = round(math.sqrt(s/math.pi), 2) #半径
            return u, v, s, r_dot
    return 0, 0, 0.0, 0.0

def main_loop(cap, threshold, params):
    IMAGE_NAMES = ['Original image', 'Gray image', 'Binary image']
    image_mode = 0
    CAMERA_WIDTH, CAMERA_HEIGHT = cap.get(cv2.CAP_PROP_FRAME_WIDTH), cap.get(cv2.CAP_PROP_FRAME_HEIGHT)

    recording = False

    while True:
        # キーボード入力を受付
        key = cv2.waitKey(1) & 0xFF
        if key == ord('q'):
            # 終了
            break
        elif key == ord('t'):
            # 表示形式変更 (カラー/グレースケール/2値)
            cv2.destroyWindow(IMAGE_NAMES[image_mode])
            image_mode = (image_mode + 1) % len(IMAGE_NAMES)
        elif key == ord('s'):
            if not recording:
                print('start recording')
                # バッファーを初期化
                buffer = Buffer(['u', 'v'])
                recording = True
            else:
                print('saving finished')
                # バッファーの内容を保存
                buffer.dump()
                recording = False
        # TODO (他のキー操作)
        
        ret, frame = cap.read()
        gray_frame = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
        
        if threshold is not None:
            # thresholdにもとづき二値化
            _, im_th = cv2.threshold(gray_frame, threshold, 255, cv2.THRESH_BINARY)
        else:
            # 大津の二値化を使う場合
            th, im_th = cv2.threshold(gray_frame, 0, 255, cv2.THRESH_OTSU)

        binary_frame = cv2.bitwise_not(im_th)
        # 輪郭検出
        contours, hierarchy = cv2.findContours(binary_frame, cv2.RETR_LIST, cv2.CHAIN_APPROX_SIMPLE)
        # 座標計算
        u, v, s, r_dot = detect_marker(contours)
            
        # TODO (x, y, zの計算）
        # 分母の０で，プログラムがフリーズになることに注意
        L, fx, fy, f = params['L'], params['fx'], params['fy'], params['f'],
        #
        
            
        if recording:
            # 時刻や座標をバッファに保存
            buffer.append(u=u, v=v)
            pass
        
        if image_mode == 0:
            show_img = frame
            point_color = (0, 200, 0)
        elif image_mode == 1:
            show_img = gray_frame
            point_color = 255
        elif image_mode == 2:
            show_img = binary_frame
            point_color = 128 
        
        # draw line
        cv2.line(show_img,(0,20),(640,20),(255,0,0),1)
        cv2.line(show_img,(60,0),(60,320),(255,0,0),1)

        # show detection info
        cv2.circle(show_img, (u, v), 6, point_color, -1)
        text = f'({u}, {v}), r: {r_dot}, num_contours: {len(contours)}'
        cv2.putText(show_img, text, (u + 20, v+ 20),
                   cv2.FONT_HERSHEY_PLAIN, 1.0,
                   (255, 255, 255), 1, cv2.LINE_AA)

        cv2.imshow(IMAGE_NAMES[image_mode], show_img)
    cv2.destroyAllWindows()
    
if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser(description="Marker detection using a camera.")
    parser.add_argument('--camera_id', type=int, default=0, help='Camera index')
    parser.add_argument('--fps', type=int, default=30, help='Frames per second')
    parser.add_argument('--camera_width', type=int, default=640, help='Frame width')
    parser.add_argument('--camera_height', type=int, default=480, help='Frame height')
    parser.add_argument('--threshold', type=int, help='Thresohld for binarization')
    
    parser.add_argument('--L', type=float, help='L')
    parser.add_argument('--fx', type=float, help='fx')
    parser.add_argument('--fy', type=float, help='fy')
    parser.add_argument('--f', type=float, help='f')
    args = parser.parse_args()

    params = {
        'L': args.L,
        'fx': args.fx,
        'fy': args.fy,
        'f': args.f,
    }
    cap = prepare_camera(camera_id=args.camera_id, fps=args.fps, width=args.camera_width, height=args.camera_height)
    main_loop(cap, args.threshold, params)
    cap.release()
