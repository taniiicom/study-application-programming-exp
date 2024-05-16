clear all; close all; 
nclass=10; % number of classes
load('./usps_resampled/usps_resampled.mat'); 
[d,ndata]=size(test_patterns);
test=zeros(d,ndata); 
trai=zeros(d,ndata); 
test_label=zeros(ndata,1);
trai_label=zeros(ndata,1); 
for ii = 1 : ndata 
    test_label(ii)=find(test_labels(:,ii)==1)-1; % 0����9�܂ł̐���
    tmp=reshape(test_patterns(:,ii),[16 16]);    % 1�����z�񂩂�16�~16��2�����z��ɕϊ�
    tmp=tmp';             % �摜���������ɕϊ�
    tmp=tmp-min(tmp(:));  % �e��f�l�����f�l�̍ŏ��l�������D����őS��f�l��0�ȏ�
    tmp=tmp./max(tmp(:)); % �e��f�l�����f�l�̍ő�l�Ŋ���D����őS��f�l��1�ȉ�
    test(:,ii)=tmp(:); % 1�����z��ɕϊ�����test�̑�ii��Ɋi�[
    trai_label(ii)=find(train_labels(:,ii)==1)-1; 
    tmp=reshape(train_patterns(:,ii),[16 16]); 
    tmp=tmp'; 
    tmp=tmp-min(tmp(:)); 
    tmp=tmp./max(tmp(:)); 
    trai(:,ii)=tmp(:); 
end
save('-mat','./usps_resampled/usps.mat','test','trai','test_label','trai_label');