% Weighted Subspace Classifier 
% clear all; close all; 

% �p�����[�^�̐ݒ�
% r=10;      % ������Ԃ̎���
imgnum=1;  % test sample number for displaying Uc 
nclass=10; % �S�N���X(0����9�̐���)�̑���

% loading data-file 
load('./usps_resampled/usps.mat'); 
[d,ndata]=size(trai);
for ii = 1 : ndata 
    trai(:,ii)=trai(:,ii)./norm(trai(:,ii)); % ���K�� 
end

% figure(1),clf
%% forming subspaces
for c = 1 : nclass 
    X=trai(:,find(trai_label==c-1)); 
    [C(c).U,C(c).eigval]=EVD(X); 
    %% displaying U of each class 
    % for ii = 1 : 10 
    %     IMG=reshape(C(c).U(:,ii),[16 16]); 
    %     IMG=IMG-min(IMG(:)); 
    %     IMG=IMG./max(IMG(:));
    %     figure(1),subplot(10,10,(c-1).*10+ii),imshow(IMG); 
    % end
    fprintf(1,'class %d ...OK\n',c-1); 
end

% figure(2),clf
%% displaying Uc
% Q=test(:,imgnum)./norm(test(:,imgnum)); 
% IMG=reshape(Q,[16 16]);
% IMG=IMG-min(IMG(:));
% IMG=IMG./max(IMG(:));
% figure(2),subplot(2,10,5),imshow(IMG); 
% title('test sample'); 
% for c = 1 : nclass 
%     a=C(c).U(:,1:r)'*Q; 
%     IMG=reshape(C(c).U(:,1:r)*a,[16 16]); 
%     IMG=IMG-min(IMG(:));
%     IMG=IMG./max(IMG(:)); 
%     figure(2),subplot(2,10,10+c),imshow(IMG); 
%     s=sprintf('class %d',c-1);
%     title(s); 
% end


%% weighted subspace classifier
w=sqrt([r:-1:1]');  % linear weight 
S=zeros(nclass,1);  % �e�N���X�̕�����ԂɎˉe�����x�N�g���̏d�ݕt���m�������i�[���邽�߂�10�~1�̔z��
CONF=zeros(nclass); % �����s��̂��߂�10�~10�̔z��
tic
for ii = 1 : ndata 
    test(:,ii)=test(:,ii)./norm(test(:,ii)); % ���K��
    for c = 1 : nclass 
        S(c)=norm(w.*(C(c).U(:,1:r)'*test(:,ii))); % �N���X�̕�����ԂɎˉe�����x�N�g���̏d�ݕt���m�������v�Z
    end
    [value,index]=max(S); %index��S(c)���ő�ƂȂ�N���Xc
    CONF(index,test_label(ii)+1)=CONF(index,test_label(ii)+1)+1; 
end
finish=toc;

accuracy=(sum(diag(CONF))./ndata).*100; 
fprintf(1,'accuracy=%3.2f\n',accuracy);
fprintf(1,'classification time per sample: %f[s]\n',finish./ndata);