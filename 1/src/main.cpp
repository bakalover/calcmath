#include<iostream>
#include <vector>
#include "../include/util.h"
using namespace std;

int main(){
    int32_t n;
    cin>>n;

    vector<vector<double>> A (n+1,vector<double> (n+1)), C (n+1,vector<double> (n+1));
    vector<double> B(n+1), d(n+1);
    for (size_t i = 1; i <= n; i++)
    {
        for (size_t j = 1; j <= n; j++)
        {
            cin>>A[i][j];
        }
    }
    
     // Диагональное преобладание

    for (size_t i = 1; i <= n; i++)
    {
        for (size_t j = 1; j <= n; j++)
        {
            if(i!=j){
                C[i][j]=-(A[i][j]/A[i][i]);
            }
        }
        
    }

    for (size_t i = 1; i <= n; i++)
    {
        cin>>B[i];
    }

    for (size_t i = 1; i <= n; i++)
    {
        d[i] = B[i]/A[i][i];
    }
    
    vector<double> x(d);
    
    int32_t iter_count = solve(C,x,d);

    cout<<iter_count<<endl;
    for (size_t i = 1; i < x.size(); i++)
    {
        cout<<x[i]<<" ";
    }
    
    return 0;

}

// 3
// 10 1 1
// 2 10 1
// 2 2 10
// 12 13 14
