import java.io.IOException;
import java.io.InputStreamReader;
import java.io.BufferedReader;
import java.io.IOException;
import java.util.*;
import java.lang.Math;
import java.lang.NullPointerException;


// 9x23열이면 8x8이 가능한 경우의수를 싹다 구해서 본 다음, 최소로 색칠하는 경우의 수를 구한다.
public class 체스판다시칠하기 {
    public static boolean[][] ChessColor; //참이면 W, 아니면B
    public static int min = 64;
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
        StringTokenizer st = new StringTokenizer(br.readLine());

        int M = Integer.parseInt(st.nextToken()); // 행
        int N = Integer.parseInt(st.nextToken()); // 열

        ChessColor = new boolean[M][N];

        for(int i = 0; i < M; i++){
            String str = br.readLine();
            str = str.toUpperCase();

            for(int j = 0; j < N; j++){
                if(str.charAt(j) == 'W'){
                    ChessColor[i][j] = true;
                }
                else{
                    ChessColor[i][j] = false;
                }
            }
        }

        int M_row = M - 7;
        int N_col = N - 7;

        for(int k = 0; k < M_row; k++){
            for(int l = 0; l < N_col; l++){
                findSketch(k,l);
            }
        }

        System.out.println(min);


    }

    public static void findSketch(int x, int y){
        int endPointX = x + 8;
        int endPointY = y + 8;
        int count = 0;
        boolean WB = ChessColor[x][y];

        for (int i = x; i < endPointX; i++){
            for (int j = y; j < endPointY; j++){

                if(ChessColor[i][j] != WB){
                    count++;
                }

                WB = !WB; //다음 거 비교하려면 앞에거랑은 다른 색이여야되니까.
            }

            WB = !WB; //체스판 성질 --> 모든 행의 끝지점과 다음 행 첫지점의 색은 항상 같다.
        }


        count = Math.min(count, 64-count);
        min = Math.min(min, count);

    }
}

