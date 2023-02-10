#include "../include/util.h"
#include <cmath>
bool check_ans(const vector<double>& x_k, const vector<double>& x_k_1, const double eps){
    for (size_t i = 1; i <= x_k.size()-1; i++)
    {
        if(abs(x_k[i] - x_k_1[i]) > eps) return false;
    }
    return true;
}

void calculate_x_1(vector<double>& x_1, vector<double>& x, const vector<vector<double>>& C, const vector<double>& d){
    for (size_t i = 1; i < x_1.size(); i++)
    {
        x_1[i] = d[i];
        for (size_t j = 1; j <=(i-1); j++)
        {
            x_1[i]+=C[i][j]*x_1[j];
        }
        for (size_t j = i; j <= x_1.size()-1; j++)
        {
            x_1[i]+=C[i][j]*x[j];
        }
        
    }
}

int32_t solve(const vector<vector<double>>& C, vector<double>& x,const vector<double>& d){
   
    int32_t iter_count{};
    vector<double> x_1(x.size(),100000);

    while(!check_ans(x,x_1,0.0000000001)){
        for (size_t i = 1; i < x.size(); i++)
        {
            cout<<x[i]<<" ";
        }
        cout<<'\n';
        ++iter_count;
        vector<double> x_1_cp(x_1);
        calculate_x_1(x_1,x,C,d);
        x = x_1_cp;
    }
    return iter_count;
}