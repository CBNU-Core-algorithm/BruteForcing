import java.io.IOException;
import java.io.InputStreamReader;
import java.io.BufferedReader;
import java.io.IOException;
import java.util.*;
public class 주사위 {
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
        StringTokenizer st = new StringTokenizer(br.readLine());

        int S1 = Integer.parseInt(st.nextToken());
        int S2 = Integer.parseInt(st.nextToken());
        int S3 = Integer.parseInt(st.nextToken());
        int[] sum = new int[S1+S2+S3-2];
        int max = sum[0];
        int maxIndex = 0;

        for(int i = 0; i < S1; i++){
            for(int j = 0; j < S2; j++){
                for(int k = 0; k < S3; k++){
                    sum[i+j+k] += 1;
                }

            }
        }

        for(int l = 0; l < sum.length; l++){
            if(sum[l] > max){
                max = sum[l];
                maxIndex = l;
            }
        }
        System.out.println(maxIndex + 3);


    }
}
