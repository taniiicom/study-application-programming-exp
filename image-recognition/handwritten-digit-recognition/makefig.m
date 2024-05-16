% Compute & Plot Accuracy
MaxDim=20;
Accuracy=zeros(1,MaxDim);
for r=1:MaxDim
 r
 WSC
 Accuracy(r)=accuracy;
end
figure
Dimension=1:MaxDim;
plot(Dimension,Accuracy,'-x')
xlabel('Dimension r')
ylabel('Accuracy[%]')