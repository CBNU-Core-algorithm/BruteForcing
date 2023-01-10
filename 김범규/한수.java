import java.io.IOException;
import java.io.InputStreamReader;
import java.io.BufferedReader;
import java.util.*;

//한수 = 각 자리수가 등차수열인 수, >>> 즉, 1,2자리 수는 모조리 한수
public class 한수 {
    public static void main(String[] args) throws IOException {
        int count = 0;
        int k = 0;
        int[] han = new int[3]; //어짜피 비교는 3자리니까.
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));

        int N = Integer.parseInt(br.readLine());

        for(int i = 1; i <= N; i++){ //N이 한수면 본인이..인식이 안된다.
            if(i < 100){
                count++;
            }
            else{
                k = i;
                han[0] = k % 10;
                k /= 10;
                han[1] = k % 10;
                k /= 10;
                han[2] = k;
                if(han[2] - han[1] == han[1] - han[0]){
                    count++;
                }
            }
        }

        System.out.println(count);


    }
}
