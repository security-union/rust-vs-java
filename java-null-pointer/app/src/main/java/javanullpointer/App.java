/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package javanullpointer;
import java.util.*;  

public class App {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        String a = (sc.nextLine() == "crash") ? null : "no crash";
        sc.close();
        System.out.println(a.length());
    }
}
