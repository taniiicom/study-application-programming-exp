% k-Nearest Neighbor Classifier（k-最近傍分類器）

% 訓練データを使ってk-最近傍分類モデル（以下モデルと略す）を構築
% k = 1; % 最近傍の数
knn_model = fitcknn(trai_data, trai_label, 'NumNeighbors', k);

% モデルの交差確認（訓練データの分割数はデフォルトの10）
cv_knn_model = crossval(knn_model);
L = kfoldLoss(cv_knn_model); % 推定誤識別率

% テストデータによるモデルの評価
tic
predict_label = predict(knn_model, test_data); % モデルを使ってテストデータを分類
T3 = toc;
%accuracy = sum(test_label == predict_label)/ndata % 識別率
misclass_rate = sum(test_label ~= predict_label)/ndata; % 誤識別率
%C = confusionmat(test_label, predict_label); % 混同行列の計算
%confusionchart(test_label, predict_label); % 混同行列を混同行列チャートとしてプロット

% 計算結果を出力
fprintf(1,'misclassification rate (CV): %2.3f\n', L*100);
fprintf(1,'misclassification rate (test data): %2.3f\n', misclass_rate*100);
fprintf(1,'classification time per sample: %f[s]\n', (T1 + T2 + T3)/ndata);

out_misclass_rate_cv = L*100;
out_misclass_rate_testdata = misclass_rate*100;
out_classification_time = (T1 + T2 + T3)/ndata;