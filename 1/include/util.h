#ifndef UTIL_HEADER
#define UTIL_HEADER

#include<iostream>
#include<vector>
#include<cinttypes>

using namespace std;
int32_t solve(const vector<vector<double>>& C, vector<double>& x,const vector<double>& d);
bool check_diag(vector<vector<double>>& A, vector<double>& b);
#endif