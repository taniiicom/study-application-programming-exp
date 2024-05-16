% 課題 1 用スクリプト

clear all

out_misclass_rate_cvs = zeros(1, 10)
out_misclass_rate_testdatas = zeros(1, 10)
out_classfication_times = zeros(1, 10)

Preprocessing
FE % mode = 0
for k = 1:10
    k
    KNNC
    out_misclass_rate_cvs(k) = out_misclass_rate_cv
    out_misclass_rate_testdatas(k) = out_misclass_rate_testdata
    out_classification_times(k) = out_classification_time
end

figure
k=1:10;
plot(k, out_misclass_rate_cvs, '-x', "DisplayName", "cv")
hold on;
plot(k, out_misclass_rate_testdatas, '-x', "DisplayName", "testdata")
xlabel('k')
ylabel('misclass rate [%]')
legend show;  % 凡例を表示
