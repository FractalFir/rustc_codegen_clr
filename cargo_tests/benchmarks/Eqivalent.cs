using System;
using System.Diagnostics;
class Test{
    public static ulong fibonacci(ulong n){
    switch (n) {
        case 0:
            return 1;
        case 1:
            return 1;
        default:
            return fibonacci(n-1) + fibonacci(n-2);
    }
}
    public static void Run(){
        fibonacci(10);
    }
    public static void Main(string[] args){
        for(int i = 0; i< 100_000/100; i++){
            Run();
        }
        var stopwatch = new Stopwatch();
        stopwatch.Start();
        for(int i = 0; i< 100_000; i++){
           Run();
        }
        stopwatch.Stop();
        var ms = stopwatch.ElapsedMilliseconds;
        var ns = (double)(ms*1_000_000);
        var ns_per_iter = ns/((double)(100_000));
        Console.WriteLine(ns_per_iter);
    }
}
