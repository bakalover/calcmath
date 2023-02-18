#ifndef UTIL_HEADER
#define UTIL_HEADER

#include<iostream>
#include<vector>
#include<cinttypes>
#include<sstream>
#include<iomanip>
using namespace std;
int32_t solve(const vector<vector<double>>& C, vector<double>& x,const vector<double>& d, const int64_t eps);
bool check_diag(vector<vector<double>>& A, vector<double>& b);
string pretty(double x, const int64_t eps);
#endif