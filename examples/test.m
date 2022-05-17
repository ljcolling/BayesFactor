% Make use BayesFactor.mexmaci64 is in path

times = 10000;
d = repmat(0.8, 1, times);
n = repmat(10, 1, times);
location = repmat(0,1, times);
scale = repmat(sqrt(2)/2, 1,times);
ll = repmat(0.5,1, times);
ul = repmat(Inf,1, times);

[bf,bf_comp] = BayesFactor(d', n', location', scale', ll', ul');
