#include "../include/util.hpp"
#include <cmath>
string pretty(double x, const int64_t eps){
    ostringstream strs;
    strs << scientific << setprecision(100) << x;
    string s =  strs.str(); // Просто циклом посчитать нули после этой строчки
    string mantis = s.substr(0,1) + s.substr(2,eps);
    int16_t exp = stoi(s.substr(s.size()-2,2)) - 1;
    if (exp >= 0){
        for (size_t i = 0; i < exp; i++)
        {
            mantis = "0"+mantis;
        }
        return "0." + mantis;
    }
    else{
        return mantis.substr(0,1) + "." + mantis.substr(1, mantis.size()-1);
    }
}

bool check_ans(const vector<double>& x_k, const vector<double>& x_k_1, const double eps, vector<double>& err){
    double max_cur_err{};
    for (size_t i = 1; i <= x_k.size()-1; i++)
    {
        max_cur_err = max(abs(x_k[i] - x_k_1[i]), max_cur_err);
        err[i] = abs(x_k[i] - x_k_1[i]);
    }
    if(max_cur_err > eps){
        return false;
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

int32_t solve(const vector<vector<double>>& C, vector<double>& x,const vector<double>& d, const int64_t eps){
   
    int32_t iter_count{};
    vector<double> err(x.size());
    vector<double> x_1(x.size(),100000);

    calculate_x_1(x_1,x,C,d);

    while(!check_ans(x,x_1,(double)1/pow(10,eps),err)){
        ++iter_count;
        x = x_1;
        calculate_x_1(x_1,x,C,d);
    }
    cout<<"\nERR: "<<endl;
    for(size_t i = 1; i< err.size();++i){
        //cout<<err[i]<<endl;
        cout << pretty(err[i], eps) << endl;
    }
    cout<<'\n';
    return ++iter_count;
}

bool check_diag(vector<vector<double>>& A, vector<double>& b){

    vector<int32_t> indexes(A.size());
    bool strict = false;

    for (size_t i = 1; i < A.size(); i++)
    {
        bool check_row = false;
        double sum{};
        for (size_t j = 1; j < A.size(); j++)
        {
            sum+=A[i][j];
        }
        
        for (size_t j = 1; j < A.size(); j++)
        {
            if(A[i][j] >= sum - A[i][j]){                    //Если такой элемент существует -> он , ((очевидно)), единственнен 
                check_row = true;
                if(A[i][j] > sum - A[i][j]) {strict = true;} // Хотя бы один должен быть строго больше
                indexes[j] = i;                              // j-й элемент в i-й строке - нужный -> j-ой строкой будет i-я строка
            }   
        }
        if(!check_row){return false;}
    }
    for(size_t i = 1; i<indexes.size()-1;++i){
        swap(b[i], b[indexes[i]]);
        for (size_t j = 1; j < A.size(); j++)
        {
            swap(A[i][j],A[indexes[i]][j]);
        }
        
    }
    double sum{};
    for (size_t i = 1; i < indexes.size(); i++)
    {
        sum += indexes[i];
    }
    if(sum - (A.size()-1)*A.size()/2 > 0.000001 || !strict) {
        return false;
    }


    return true;
}