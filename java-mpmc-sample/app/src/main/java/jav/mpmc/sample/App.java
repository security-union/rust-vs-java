package jav.mpmc.sample;

import java.util.ArrayList;
import java.util.concurrent.ConcurrentLinkedQueue;

public class App {
    public static void main(String[] args) throws InterruptedException {
        // Create a concurrent linked queue to hold the messages
        final var queue = new ConcurrentLinkedQueue<Integer>();
        
        // Spawn producer threads
        for (int i = 0; i < 5; i++) {
            final var acc = i;
            new Thread(() -> {
                    queue.add(acc *2);
            }).start();
        }
        // Spawn consumer threads
        var handles = new ArrayList<Thread>();
        for (int i = 0; i < 5; i++) {
            var thread = new Thread(() -> {
                while (true) {
                    var message = queue.poll();
                    if (message != null) {
                        System.out.println("received value " + message);
                        return;
                    }
                }
            });
            thread.start();
            handles.add(thread);
        }
        // iterate over all threads and wait for them to finish
        for (var handle : handles) {
            handle.join();
        }
    }
}
