#include "mex.h"

extern void bfwrapper(double *d, double *n, double *location, double *scale,
                      double *ll, double *ul, double *bf, double *bf_comp,
                      long elements);

void mexFunction(int nlhs, mxArray *plhs[], int nrhs, const mxArray *prhs[]) {
  double *d;
  double *n;
  double *location;
  double *scale;
  double *ll;
  double *ul;
  double *bf;
  double *bf_comp;

  mwSize elements;

  if (nrhs != 6) {
    mexErrMsgTxt("Wrong number of input args");
  }

  // the inputs
  d = mxGetPr(prhs[0]);
  n = mxGetPr(prhs[1]);
  location = mxGetPr(prhs[2]);
  scale = mxGetPr(prhs[3]);
  ll = mxGetPr(prhs[4]);
  ul = mxGetPr(prhs[5]);

  elements = mxGetM(prhs[0]);

  // the outputs
  plhs[0] = mxCreateDoubleMatrix(elements, 1, mxREAL);
  plhs[1] = mxCreateDoubleMatrix(elements, 1, mxREAL);
  bf = mxGetPr(plhs[0]);
  bf_comp = mxGetPr(plhs[1]);

  // run the function
  bfwrapper(d, n, location, scale, ll, ul, bf, bf_comp, elements);
}

