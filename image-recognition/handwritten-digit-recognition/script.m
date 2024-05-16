% Compute & Plot Accuracy

MaxDim=20;  % MaxDimの定義を先頭に移動
Accuracy=zeros(1,MaxDim);
for r=1:MaxDim
    r
    WSC
    Accuracy(r)=accuracy;
end

figure
Dimension=1:20;
plot(Dimension,Accuracy,'-x')
xlabel('次元 r')
ylabel('認識率[%]')
