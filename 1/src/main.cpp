#include<iostream>
#include <vector>
#include "../include/util.hpp"
#include <iomanip>
#include <fstream>
using namespace std;

int main(){
    cout<<setprecision(30)<<fixed;
    int32_t n;
    int64_t eps;
    ifstream in;

    cout<<"Choose configuration: \n1 - console \n2 - file\n"<<endl;
    int16_t conf;
    cin>>conf;
    if(conf==2){
        cout<<"Enter path: ";

        string path;
        cin>>path;

        in.open(path);
        if(in.is_open()){
            in>>n;
        }
        else{
            cout<<"Wrong file"<<endl;;
            return 0;
        }
    }
    else{
        cout<<"\nDemensions: ";
        cin>>n;
    }

    vector<vector<double>> A (n+1,vector<double> (n+1)), C (n+1,vector<double> (n+1));
    vector<double> B(n+1), d(n+1);

    if(conf==2){
        for (size_t i = 1; i <= n; i++)
        {
            for (size_t j = 1; j <= n; j++)
            {
                in>>A[i][j];
            }
        }
        for (size_t i = 1; i <= n; i++)
        {
            in>>B[i];
        }
        in>>eps;
    }
    else{
        cout<<"\nMatrix: "<<endl;;
        for (size_t i = 1; i <= n; i++)
        {
            for (size_t j = 1; j <= n; j++)
            {
                cin>>A[i][j];
            }
        }
        cout<<"\nVector: "<<endl;
        for (size_t i = 1; i <= n; i++)
        {
            cin>>B[i];
        }
        cout<<"\nPrecision: ";
        cin>>eps;
    }

    if(!check_diag(A,B)){
        cout<<"Diagonal dominating is  absent!"<<endl;
        return 0;
    }

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
        d[i] = B[i]/A[i][i];
    }
    
    vector<double> x(d);
    
    int32_t iter_count = solve(C,x,d,eps);

    cout<<"Iterations: "<<iter_count<<'\n'<<'\n'<<"Answer: "<<endl;
    for (size_t i = 1; i < x.size(); i++)
    {
        cout<<pretty(x[i],5)<<endl;
    }
    
    return 0;

}

// 3
// 10 1 1
// 2 10 1
// 2 2 10
// 12 13 14

// 3
// 2 2 10
// 10 1 1
// 2 10 1
// 14 12 13
// 2