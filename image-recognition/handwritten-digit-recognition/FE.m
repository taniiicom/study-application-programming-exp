% Feature Extraction（特徴抽出）

% モード設定
mode = 0; % 0: HOG特徴抽出なし 1: HOG特徴抽出あり

% HOGパラメータの設定
cellSize = [4 4]; % [2 2], [4 4], [8 8]
%clear test_data trai_data % cellSizeの値を変更する場合は「%」を外す

switch mode
    case 0
        trai_data = trai'; % 訓練データ
        test_data = test'; % テストデータ
        T2 = 0;
    case 1
        for ii = 1 : ndata 
            img = reshape(trai(:,ii), [16 16]); 
            trai_data(ii,:) = extractHOGFeatures(img, 'CellSize', cellSize);
        end
        tic
        for ii = 1 : ndata 
            img = reshape(test(:,ii), [16 16]);
            test_data(ii,:) = extractHOGFeatures(img, 'CellSize', cellSize);
        end
        T2 = toc;
end
