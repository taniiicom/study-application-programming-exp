% HOG test（正規化後の画像のHOG特徴を確認）

ii=1;
img=reshape(trai(:,ii), [16 16]); % 長さ256の1次元配列を16×16の2次元配列に変換
subplot(1,2,1); imshow(img);

cellSize = [2 2]; % [2 2], [4 4], [8 8]
[hog, vis] = extractHOGFeatures(img, 'CellSize', cellSize); % HOG特徴を抽出
subplot(1,2,2); plot(vis);
