% k-Nearest Neighbor Classifier�ik-�ŋߖT���ފ�j

% �P���f�[�^���g����k-�ŋߖT���ރ��f���i�ȉ����f���Ɨ����j���\�z
% k = 1; % �ŋߖT�̐�
knn_model = fitcknn(trai_data, trai_label, 'NumNeighbors', k);

% ���f���̌����m�F�i�P���f�[�^�̕������̓f�t�H���g��10�j
cv_knn_model = crossval(knn_model);
L = kfoldLoss(cv_knn_model); % ����뎯�ʗ�

% �e�X�g�f�[�^�ɂ�郂�f���̕]��
tic
predict_label = predict(knn_model, test_data); % ���f�����g���ăe�X�g�f�[�^�𕪗�
T3 = toc;
%accuracy = sum(test_label == predict_label)/ndata % ���ʗ�
misclass_rate = sum(test_label ~= predict_label)/ndata; % �뎯�ʗ�
%C = confusionmat(test_label, predict_label); % �����s��̌v�Z
%confusionchart(test_label, predict_label); % �����s��������s��`���[�g�Ƃ��ăv���b�g

% �v�Z���ʂ��o��
fprintf(1,'misclassification rate (CV): %2.3f\n', L*100);
fprintf(1,'misclassification rate (test data): %2.3f\n', misclass_rate*100);
fprintf(1,'classification time per sample: %f[s]\n', (T1 + T2 + T3)/ndata);

out_misclass_rate_cv = L*100;
out_misclass_rate_testdata = misclass_rate*100;
out_classification_time = (T1 + T2 + T3)/ndata;