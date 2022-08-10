#include <bits/stdc++.h>

typedef unsigned char uchar;
typedef unsigned short ushort;
typedef unsigned int uint;
typedef unsigned long ulong;
typedef long long ll;
typedef unsigned long long ull;
typedef long double ld;

#define all(x) x.begin(), x.end()

using namespace std;

//Debugging macros (used like 'db("this is a debugging message")'
#ifdef DEBUG
    #define db(X) cerr << '[' << __func__ << "() | " << __LINE__ << ']' << " > Debug: " << (X) << '\n';
    #define dbstr(X) cerr << '[' << __func__ << "() | " << __LINE__ << ']' << " > Debug: " << (#X) << '\n';
#else
    #define db(X)
    #define dbstr(X) 
#endif



// >> SOLUTION HERE <<
void solve()
{

}











int main()
{
    //Decouple C++ streams from C I/O (scanf, printf, ...) for increased speed
    ios::sync_with_stdio(false);
    cin.tie(NULL);
    cout.tie(NULL);


    //Solve problem (with runtime analysis in debug mode)
    #ifdef DEBUG
        cout << fixed << setprecision(2);
        chrono::high_resolution_clock::time_point start = chrono::high_resolution_clock::now();
        solve();
        chrono::high_resolution_clock::time_point end = chrono::high_resolution_clock::now();
        chrono::duration<double,milli> elapsed_time = end - start;
        cout << "\n[DEBUG] Finished program in " << elapsed_time.count() << "ms\n";
    #else
        solve();
    #endif


    return 0;
}
