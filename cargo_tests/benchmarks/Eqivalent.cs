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
    public static void RunFib(){
        fibonacci(10);
    }
  
    public static void FibBench(){
        for(int i = 0; i< 100_000_000; i++){
            RunFib();
        }
        var stopwatch = new Stopwatch();
        stopwatch.Start();
        for(int i = 0; i< 100_000_000; i++){
           RunFib();
        }
        stopwatch.Stop();
        var ms = stopwatch.ElapsedMilliseconds;
        var ns = (double)(ms*1_000_000);
        var ns_per_iter = ns/((double)(100_000_000));
        Console.WriteLine(ns_per_iter);
    }
   
    public static void Main(string[] args){
        FibBench();
  
    }
}
