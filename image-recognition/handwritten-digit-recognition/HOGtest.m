% HOG test�i���K����̉摜��HOG�������m�F�j

ii=1;
img=reshape(trai(:,ii), [16 16]); % ����256��1�����z���16�~16��2�����z��ɕϊ�
subplot(1,2,1); imshow(img);

cellSize = [2 2]; % [2 2], [4 4], [8 8]
[hog, vis] = extractHOGFeatures(img, 'CellSize', cellSize); % HOG�����𒊏o
subplot(1,2,2); plot(vis);
