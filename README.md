# rust-vs-java

   <h1 class="c21 c9" id="h.haiicng23nto"><span class="c13">Java vs Rust Cheat Sheet</span></h1>
    <p class="c2"><span class="c0"></span></p><a id="t.74408c811cb72246144fce2a0770e523b9a671d4"></a><a id="t.0"></a>
    <table class="c38">
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Dimension</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Java</span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Rust</span></p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Language Goals</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <ol class="c28 lst-kix_dhl10bsr8cco-0 start" start="1">
                    <li class="c7 c11 li-bullet-0"><span class="c15">Memory Safety</span><span
                            class="c0">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</span></li>
                    <li class="c7 c11 li-bullet-0"><span class="c15">Portability</span><span class="c0">&nbsp;(compile
                            once run anywhere via the JVM)</span></li>
                    <li class="c7 c11 li-bullet-0"><span class="c0">Ease of use (coming from C)</span></li>
                </ol>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <ol class="c28 lst-kix_1sqj4wtxvx4k-0 start" start="1">
                    <li class="c7 c11 li-bullet-0"><span class="c22 c15 c37">Memory Safety</span></li>
                    <li class="c7 c11 li-bullet-0"><span class="c15">Portability</span><span class="c0">&nbsp;(compile
                            for each platform via LLVM)</span></li>
                    <li class="c7 c11 li-bullet-0"><span class="c0">Efficiency</span></li>
                    <li class="c7 c11 li-bullet-0"><span class="c0">Replace C and C++.</span></li>
                </ol>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Safety and Memory Management</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">Garbage collector.</span></p>
                <p class="c4 c9"><span class="c0"></span></p>
                <p class="c7 c9"><span class="c0">Possible to suffer from memory-related issues such as memory leaks,
                        garbage collection pauses, or poor performance due to poor memory usage patterns. </span></p>
                <p class="c4 c9"><span class="c0"></span></p>
                <p class="c7 c9"><span>May require careful tuning and optimization of the program and the garbage
                        collector to achieve good performance.</span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">Mostly automatic, enforced by the borrow checker.</span></p>
                <p class="c4"><span class="c0"></span></p>
                <p class="c7"><span class="c0">The borrow checker is a static analysis tool that ensures safe and
                        correct use of memory by checking that references to data are valid and that data is not
                        borrowed mutably while it is also borrowed immutably.</span></p>
                <p class="c4"><span class="c0"></span></p>
                <p class="c7"><span class="c0">Developers still need to think carefully about the lifetime of
                        values.</span></p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c12">Compilation (</span><span class="c0">look at tools for specific
                        tools)</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c3"><span class="c0">Generates bytecode.</span></p>
                <p class="c2"><span class="c0"></span></p>
                <p class="c3"><span class="c0">Bytecode is executed by the Java Virtual Machine (JVM), which is mostly a
                        platform-independent execution environment.</span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c3"><span class="c0">Generates native machine code that can be run on the target
                        platform.</span></p>
                <p class="c4"><span class="c0"></span></p>
                <p class="c7"><span class="c0">Uses LLVM as the compiler backend (just like Swift).</span></p>
            </td>
        </tr>
        <tr class="c36">
            <td class="c29" colspan="3" rowspan="1">
                <p class="c7"><span class="c5">Syntax</span></p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Variables mutability</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">Mutable by default, unless final is used. </span></p>
                <p class="c7"><span class="c0">var javaNumber = 3;</span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">Immutable by default, use mut to make it mutable.</span></p>
                <p class="c7"><span class="c0">let mut rust_str = &ldquo;yolo&rdquo;; </span></p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Structure</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">Classes are templates for objects that define behavior (methods) and
                        state (properties).</span></p>
                <p class="c4"><span class="c0"></span></p>
                <p class="c7"><span>Composition is possible via </span><span class="c12">interfaces.</span></p>
                <p class="c4"><span class="c5"></span></p>
                <p class="c7"><span>Class inheritance is implemented using </span><span class="c12">extends.</span><span
                        class="c0">&nbsp;</span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">Structs are used mostly to store state (properties), traits and stand
                        alone functions are used to define behavior.</span></p>
                <p class="c4"><span class="c0"></span></p>
                <p class="c7"><span class="c0">Composition is preferred and encouraged via traits on structs and
                        enums.</span></p>
                <p class="c4"><span class="c0"></span></p>
                <p class="c7"><span class="c0">Struct inheritance is not possible, instead, use trait
                        inheritance.</span></p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Generics</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">public class List&lt;T extends Comparable&lt;T&gt;&gt; {</span></p>
                <p class="c7"><span class="c0">&nbsp; // ...</span></p>
                <p class="c7"><span class="c0">}</span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">struct Vec&lt;T: Ord&gt; {</span></p>
                <p class="c7"><span class="c0">&nbsp; // ...</span></p>
                <p class="c7"><span class="c0">}</span></p>
                <p class="c4"><span class="c0"></span></p>
            </td>
        </tr>
        <tr class="c36">
            <td class="c29" colspan="3" rowspan="1">
                <p class="c7"><span class="c5">Concurrency</span></p>
            </td>
        </tr>
        <tr class="c36">
            <td class="c29" colspan="3" rowspan="1">
                <p class="c7"><span class="c5">Blocking</span></p>
            </td>
        </tr>
        <tr class="c36">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Deadlocks possible?</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">Yes</span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">Yes</span></p>
            </td>
        </tr>
        <tr class="c36">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Deadlock sample</span></p>
                <p class="c4"><span class="c5"></span></p>
                <p class="c7"><span class="c0">On both samples, what causes the deadlock is that locks are acquired in a
                        different order.</span></p>
                <p class="c4"><span class="c0"></span></p>
                <p class="c7"><span class="c0">I recommend using non-blocking concurrency techniques to avoid
                        this.</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c3"><span class="c1">class Deadlock {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; static Object lock1 = new Object();</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; static Object lock2 = new Object();</span></p>
                <p class="c2"><span class="c1"></span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; public static void main(String[] args) {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; Thread t1 = new Thread(new Runnable()
                        {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; public void run() {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;
                        synchronized(lock1) {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; try
                        {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;
                        &nbsp; &nbsp; Thread.sleep(100);</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; }
                        catch (InterruptedException e) {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;
                        &nbsp; &nbsp; e.printStackTrace();</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;
                        }</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;
                        synchronized(lock2) {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;
                        &nbsp; &nbsp; System.out.println(&quot;Thread 1&quot;);</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;
                        }</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; }</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; }</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; });</span></p>
                <p class="c2"><span class="c1"></span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; Thread t2 = new Thread(new Runnable()
                        {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; public void run() {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;
                        synchronized(lock2) {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; try
                        {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;
                        &nbsp; &nbsp; Thread.sleep(100);</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; }
                        catch (InterruptedException e) {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;
                        &nbsp; &nbsp; e.printStackTrace();</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;
                        }</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;
                        synchronized(lock1) {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;
                        &nbsp; &nbsp; System.out.println(&quot;Thread 2&quot;);</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;
                        }</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; }</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; }</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; });</span></p>
                <p class="c2"><span class="c1"></span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; t1.start();</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; t2.start();</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; }</span></p>
                <p class="c3"><span class="c1">}</span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c3"><span class="c1">fn main() {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; let lock1 = Arc::new(Mutex::new(1));</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; let lock2 = Arc::new(Mutex::new(2));</span></p>
                <p class="c2"><span class="c1"></span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; let lock1_clone = lock1.clone();</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; let lock2_clone = lock2.clone();</span></p>
                <p class="c2"><span class="c1"></span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; let t1 = thread::spawn(move || {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; let _guard1 =
                        lock1_clone.lock().unwrap();</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; println!(&quot;Thread 1 acquired lock
                        1&quot;);</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp;
                        thread::sleep(std::time::Duration::from_secs(1));</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; println!(&quot;Thread 1 trying to acquire
                        lock 2&quot;);</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; let _guard2 =
                        lock2_clone.lock().unwrap();</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; println!(&quot;Thread 1 acquired lock
                        2&quot;);</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; });</span></p>
                <p class="c2"><span class="c1"></span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; let t2 = thread::spawn(move || {</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; let _guard2 = lock2.lock().unwrap();</span>
                </p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; println!(&quot;Thread 2 acquired lock
                        2&quot;);</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp;
                        thread::sleep(std::time::Duration::from_secs(1));</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; println!(&quot;Thread 2 trying to acquire
                        lock 1&quot;);</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; let _guard1 = lock1.lock().unwrap();</span>
                </p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; &nbsp; &nbsp; println!(&quot;Thread 2 acquired lock
                        1&quot;);</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; });</span></p>
                <p class="c2"><span class="c1"></span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; t1.join().unwrap();</span></p>
                <p class="c3"><span class="c1">&nbsp; &nbsp; t2.join().unwrap();</span></p>
                <p class="c3"><span class="c1">}</span></p>
            </td>
        </tr>
        <tr class="c36">
            <td class="c29" colspan="3" rowspan="1">
                <p class="c7"><span class="c12">Non blocking</span></p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Parallel collections</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7"><span>Streams API </span><span class="c10"><a class="c8"
                            href="https://www.google.com/url?q=https://docs.oracle.com/javase/tutorial/collections/streams/parallelism.html&amp;sa=D&amp;source=editors&amp;ust=1672208416876128&amp;usg=AOvVaw1GLb7G0KyasvYNjNykel22">https://docs.oracle.com/javase/tutorial/collections/streams/parallelism.html</a></span>
                </p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7"><span>Rayon: </span><span class="c10"><a class="c8"
                            href="https://www.google.com/url?q=https://crates.io/crates/rayon&amp;sa=D&amp;source=editors&amp;ust=1672208416876564&amp;usg=AOvVaw3jxkb-1KZdDbEe-YNLuhA7">https://crates.io/crates/rayon</a></span>
                </p>
                <p class="c4"><span class="c0"></span></p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Atomic objects</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7 c9"><span>The specifications of these methods enable implementations to employ efficient
                        machine-level atomic instructions that are available on contemporary processors. However on some
                        platforms, support may entail some form of internal locking. Thus the methods are not strictly
                        guaranteed to be non-blocking -- a thread may block transiently before performing the
                        operation</span></p>
                <p class="c4"><span class="c0"></span></p>
                <p class="c7"><span class="c10"><a class="c8"
                            href="https://www.google.com/url?q=https://docs.oracle.com/javase/7/docs/api/java/util/concurrent/atomic/package-summary.html&amp;sa=D&amp;source=editors&amp;ust=1672208416877510&amp;usg=AOvVaw1Htxiqi9CXku8R23Y6mkWF">https://docs.oracle.com/javase/7/docs/api/java/util/concurrent/atomic/package-summary.html</a></span>
                </p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7 c9"><span>All atomic types in this module are guaranteed to be </span><span class="c10"><a
                            class="c8"
                            href="https://www.google.com/url?q=https://en.wikipedia.org/wiki/Non-blocking_algorithm&amp;sa=D&amp;source=editors&amp;ust=1672208416877875&amp;usg=AOvVaw0T3G8Fx27pmQl2uGQMyOaC">lock-free</a></span><span>&nbsp;if
                        they&rsquo;re available. This means they don&rsquo;t internally acquire a global mutex. Atomic
                        types and operations are not guaranteed to be wait-free.</span></p>
                <p class="c4"><span class="c24"></span></p>
                <p class="c7"><span class="c10"><a class="c8"
                            href="https://www.google.com/url?q=https://doc.rust-lang.org/std/sync/atomic/&amp;sa=D&amp;source=editors&amp;ust=1672208416878199&amp;usg=AOvVaw2foDiYhch-aktUB5NVgYKt">https://doc.rust-lang.org/std/sync/atomic/</a></span>
                </p>
                <p class="c4"><span class="c0"></span></p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c12">MPMC</span><span>: Multi-producer multi-consumer</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7 c9"><span class="c10"><a class="c8"
                            href="https://www.google.com/url?q=https://docs.oracle.com/javase/7/docs/api/java/util/concurrent/ConcurrentLinkedQueue.html&amp;sa=D&amp;source=editors&amp;ust=1672208416878906&amp;usg=AOvVaw0w7Dk0SFfZQp-LYyEMUT_9">https://docs.oracle.com/javase/7/docs/api/java/util/concurrent/ConcurrentLinkedQueue.html</a></span>
                </p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7"><span class="c10"><a class="c8"
                            href="https://www.google.com/url?q=https://doc.rust-lang.org/rust-by-example/std_misc/channels.html&amp;sa=D&amp;source=editors&amp;ust=1672208416879242&amp;usg=AOvVaw0Fs-x8AwWBlkRnHGPCx-gH">https://doc.rust-lang.org/rust-by-example/std_misc/channels.html</a></span>
                </p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7 c9"><span class="c12">MPSC</span><span class="c0">:</span></p>
                <p class="c7 c9"><span>Multi-producer single consumer</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7 c9"><span class="c0">I did not find a specialized class to do this but you can use
                        MPMC.</span></p>
                <p class="c4 c9"><span class="c0"></span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7"><span class="c10"><a class="c8"
                            href="https://www.google.com/url?q=https://crates.io/crates/crossbeam-channel&amp;sa=D&amp;source=editors&amp;ust=1672208416880078&amp;usg=AOvVaw0mPbdIyW8sFAdPj3JSEQbP">https://crates.io/crates/crossbeam-channel</a></span>
                </p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7 c9"><span class="c5">Actor model and reactive programming</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7 c9"><span>Akka </span><span class="c10"><a class="c8"
                            href="https://www.google.com/url?q=https://akka.io/&amp;sa=D&amp;source=editors&amp;ust=1672208416880677&amp;usg=AOvVaw04VSWHVguOt5wEGoLw89mN">https://akka.io/</a></span>
                </p>
                <p class="c7 c9"><span><br>Akka is no longer free for commercial use: </span><span class="c10"><a
                            class="c8"
                            href="https://www.google.com/url?q=https://www.lightbend.com/akka/license-faq&amp;sa=D&amp;source=editors&amp;ust=1672208416881013&amp;usg=AOvVaw2Z05IP2RLKTCJCFE4Qqsco">https://www.lightbend.com/akka/license-faq</a></span>
                </p>
                <p class="c4 c9"><span class="c0"></span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7"><span>Actix </span><span class="c10"><a class="c8"
                            href="https://www.google.com/url?q=https://github.com/actix/actix&amp;sa=D&amp;source=editors&amp;ust=1672208416881556&amp;usg=AOvVaw23De2jeB91IxiXHnpeimXT">https://github.com/actix/actix</a></span>
                </p>
                <p class="c4"><span class="c0"></span></p>
                <p class="c7"><span>MIT License: </span><span class="c10"><a class="c8"
                            href="https://www.google.com/url?q=https://github.com/actix/actix/blob/master/LICENSE-MIT&amp;sa=D&amp;source=editors&amp;ust=1672208416881927&amp;usg=AOvVaw3MGHNRy4Fz7vYvjUxq2pvH">https://github.com/actix/actix/blob/master/LICENSE-MIT</a></span>
                </p>
                <p class="c4"><span class="c0"></span></p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">async/await</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">NONE</span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">Tokio, async-std, actix</span></p>
            </td>
        </tr>
        <tr class="c26">
            <td class="c29" colspan="3" rowspan="1">
                <p class="c7"><span class="c12">Tooling *</span></p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Compiler</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7"><span>javac</span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">rustc (backend: llvm)</span></p>
                <p class="c4"><span class="c5"></span></p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Toolchain manager</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7"><span>sdkman &nbsp;</span><span class="c10"><a class="c8"
                            href="https://www.google.com/url?q=https://sdkman.io/&amp;sa=D&amp;source=editors&amp;ust=1672208416884273&amp;usg=AOvVaw2eD_0gSYOSLJF1NllmNyWp">https://sdkman.io/</a></span><span
                        class="c0"><br></span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7"><span>cargo </span><span class="c10"><a class="c8"
                            href="https://www.google.com/url?q=https://doc.rust-lang.org/cargo/getting-started/installation.html&amp;sa=D&amp;source=editors&amp;ust=1672208416884621&amp;usg=AOvVaw2N8CrAQXk4xPzfYjzXjWeI">https://doc.rust-lang.org/cargo/getting-started/installation.html</a></span>
                </p>
                <p class="c4"><span class="c0"></span></p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Dependency Manager</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">Gradle, Maven (old)</span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">cargo</span></p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Resources needed to run a web server hello world (dev mode) **</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">Spring Boot 3.0.0</span></p>
                <p class="c7"><span class="c0">2 processes:<br>MEM: &nbsp; &nbsp; &nbsp; 134MB | 80 MB<br>CPU: &nbsp;
                        &nbsp; &nbsp; &nbsp;1.28 &nbsp; &nbsp; | 1.40 </span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7"><span class="c0">Actix-web 4.0</span></p>
                <p class="c7"><span class="c0">1 process:</span></p>
                <p class="c7"><span class="c0">MEM: 7 MB</span></p>
                <p class="c7"><span class="c0">CPU: 0.37</span></p>
                <p class="c4"><span class="c0"></span></p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Licensing</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7"><span>OracleJDK (stay away from this garbage) </span><span class="c10"><a class="c8"
                            href="https://www.google.com/url?q=https://www.oracle.com/downloads/licenses/no-fee-license.html&amp;sa=D&amp;source=editors&amp;ust=1672208416887101&amp;usg=AOvVaw1BUdtSz3vMPkkwS2FGG0Gp">https://www.oracle.com/downloads/licenses/no-fee-license.html</a></span>
                </p>
                <p class="c7"><span class="c0">OpenJDK (preferred)</span></p>
                <p class="c7"><span class="c10"><a class="c8"
                            href="https://www.google.com/url?q=https://openjdk.org/legal/gplv2%2Bce.html&amp;sa=D&amp;source=editors&amp;ust=1672208416887610&amp;usg=AOvVaw3q3zksJuHBEkFmoW0FTzXI">https://openjdk.org/legal/gplv2+ce.html</a></span>
                </p>
                <p class="c4"><span class="c0"></span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7 c9"><span class="c0">The Rust Programming Language and all other official projects are
                        dual-licensed:</span></p>
                <ul class="c28 lst-kix_rikxt4rgfq2z-0 start">
                    <li class="c7 c9 c11 li-bullet-0"><span class="c10"><a class="c8"
                                href="https://www.google.com/url?q=http://www.apache.org/licenses/LICENSE-2.0&amp;sa=D&amp;source=editors&amp;ust=1672208416888137&amp;usg=AOvVaw1DbJtVLWn58oyeqCk9GxvP">Apache
                                License, Version 2.0</a></span></li>
                    <li class="c7 c9 c11 li-bullet-0"><span class="c10"><a class="c8"
                                href="https://www.google.com/url?q=http://opensource.org/licenses/MIT&amp;sa=D&amp;source=editors&amp;ust=1672208416888402&amp;usg=AOvVaw0MjvOT8pFDxZATJXGe1bTZ">MIT
                                license</a></span></li>
                </ul>
                <p class="c4"><span class="c0"></span></p>
            </td>
        </tr>
        <tr class="c6">
            <td class="c19" colspan="1" rowspan="1">
                <p class="c7"><span class="c5">Cost</span></p>
            </td>
            <td class="c16" colspan="1" rowspan="1">
                <p class="c7 c9"><span>Oracle announced that beginning from Java JDK 17 and onwards Java is free for
                        commercial usage. &ldquo;If you are running older versions of Java 1-16, you are not affected by
                        the new licensing agreement.&rdquo; </span><span class="c15 c12 c22">Never believe
                        Oracle.</span></p>
                <p class="c4"><span class="c0"></span></p>
            </td>
            <td class="c14" colspan="1" rowspan="1">
                <p class="c7 c9"><span class="c0">Free</span></p>
                <p class="c4 c9"><span class="c0"></span></p>
                <p class="c7 c9"><span>Company donations are appreciated: </span><span class="c10"><a class="c8"
                            href="https://www.google.com/url?q=https://www.rust-lang.org/sponsors&amp;sa=D&amp;source=editors&amp;ust=1672208416889661&amp;usg=AOvVaw1RtupU84yenXcb9wpdsIw_">https://www.rust-lang.org/sponsors</a></span>
                </p>
                <p class="c4 c9"><span class="c0"></span></p>
            </td>
        </tr>
    </table>