package jav.mpmc.sample;

import java.util.ArrayList;
import java.util.concurrent.ConcurrentLinkedQueue;

public class App {
    public static void main(String[] args) throws InterruptedException {
        // Create a concurrent linked queue to hold the messages
        ConcurrentLinkedQueue<Integer> queue = new ConcurrentLinkedQueue<>();

        // Create and start 5 producer threads
        for (int i = 0; i < 5; i++) {
            final var acc = i;
            new Thread(() -> {
                    // Add a message to the queue
                    queue.add(acc *2);
            }).start();
        }

        // Create and start 5 consumer threads
        var handles = new ArrayList<Thread>();
        for (int i = 0; i < 5; i++) {
            var thread = new Thread(() -> {
                // Try to retrieve a message from the queue
                while (true) {
                    Integer message = queue.poll();
                    if (message != null) {
                        // If a message was retrieved, print it and exit
                        System.out.println("Received value " + message);
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
