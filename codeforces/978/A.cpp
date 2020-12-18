#include <iostream>
#include <iterator>
#include <vector>
#include <algorithm>

int32_t main(void){
    int n,a;
    std::vector<int> v;
    std::vector<int>::iterator it;

    std::cin>>n;

    for(int i=0; i<n; i++){
        std::cin>>a;
        it=std::find(v.begin(),v.end(),a);
        if(it!=v.end()) v.erase(it);
        v.push_back(a);
    }

    std::cout<<v.size()<<std::endl;

    for(auto x : v) std::cout<<x<<" ";

    std::cout<<std::endl;
}
