% Preprocessing（前処理）

clear all

% loading data-file 
load('./usps_resampled/usps.mat'); 
[d, ndata] = size(trai);

% 正規化処理（ベクトルの長さを1に規格化）
for ii = 1 : ndata 
    trai(:,ii) = trai(:,ii)./norm(trai(:,ii));
end
tic
for ii = 1 : ndata 
    test(:,ii) = test(:,ii)./norm(test(:,ii));
end
T1 = toc;