% Feature Extraction�i�������o�j

% ���[�h�ݒ�
mode = 0; % 0: HOG�������o�Ȃ� 1: HOG�������o����

% HOG�p�����[�^�̐ݒ�
cellSize = [4 4]; % [2 2], [4 4], [8 8]
%clear test_data trai_data % cellSize�̒l��ύX����ꍇ�́u%�v���O��

switch mode
    case 0
        trai_data = trai'; % �P���f�[�^
        test_data = test'; % �e�X�g�f�[�^
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
