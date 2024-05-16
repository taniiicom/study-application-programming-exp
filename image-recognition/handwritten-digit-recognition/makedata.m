clear all; close all; 
nclass=10; % number of classes
load('./usps_resampled/usps_resampled.mat'); 
[d,ndata]=size(test_patterns);
test=zeros(d,ndata); 
trai=zeros(d,ndata); 
test_label=zeros(ndata,1);
trai_label=zeros(ndata,1); 
for ii = 1 : ndata 
    test_label(ii)=find(test_labels(:,ii)==1)-1; % 0から9までの数字
    tmp=reshape(test_patterns(:,ii),[16 16]);    % 1次元配列から16×16の2次元配列に変換
    tmp=tmp';             % 画像を横向きに変換
    tmp=tmp-min(tmp(:));  % 各画素値から画素値の最小値を引く．これで全画素値は0以上
    tmp=tmp./max(tmp(:)); % 各画素値から画素値の最大値で割る．これで全画素値は1以下
    test(:,ii)=tmp(:); % 1次元配列に変換してtestの第ii列に格納
    trai_label(ii)=find(train_labels(:,ii)==1)-1; 
    tmp=reshape(train_patterns(:,ii),[16 16]); 
    tmp=tmp'; 
    tmp=tmp-min(tmp(:)); 
    tmp=tmp./max(tmp(:)); 
    trai(:,ii)=tmp(:); 
end
save('-mat','./usps_resampled/usps.mat','test','trai','test_label','trai_label');