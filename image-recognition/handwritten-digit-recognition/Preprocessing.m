% Preprocessing�i�O�����j

clear all

% loading data-file 
load('./usps_resampled/usps.mat'); 
[d, ndata] = size(trai);

% ���K�������i�x�N�g���̒�����1�ɋK�i���j
for ii = 1 : ndata 
    trai(:,ii) = trai(:,ii)./norm(trai(:,ii));
end
tic
for ii = 1 : ndata 
    test(:,ii) = test(:,ii)./norm(test(:,ii));
end
T1 = toc;