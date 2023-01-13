#include <iostream>
#include <vector>
#include <string> 
#include <algorithm> //조합 함수를 쓰기 위한 라이브러리
using namespace std;

//nCr = n-1Cr-1 + n-1Cr
//https://ansohxxn.github.io/algorithm/combination/
void Combination(vector<int> v, vector<int> three, int r, int index, int depth, int max_number, int& comparison, int& card_three_sum)
{
        int v_size = v.size();

        if (r == 0)
        {      
            card_three_sum = three[0] + three[1] + three[2]; //3개 카드의 합
            if ((card_three_sum > comparison) && (card_three_sum <= max_number)) { //카드 합이 비굣값보다 크고 최대수 이하라면
            comparison = card_three_sum; // 비굣값에 카드합 저장
            }
        }

        else if (depth == v_size)  // depth == n // 계속 안뽑다가 r 개를 채우지 못한 경우는 이 곳에서 걸러냄
        {
            return;
        }
        else
        {
            // arr[depth] 를 뽑은 경우
            three[index] = v[depth];
            Combination(v, three, r - 1, index + 1, depth + 1, max_number, comparison, card_three_sum);

            // arr[depth] 를 뽑지 않은 경우
            Combination(v, three, r, index, depth + 1, max_number, comparison, card_three_sum);
        }
}

int main(){

    int total_card; //총 카드의 수
    int max_number; // 카드 3합이 최대로 가질 수 있는 수

    cin >> total_card >> max_number; //입력

    vector<int> v(total_card); //각 카드의 저장공간

    for (int i = 0; i < total_card; i++) { // 저장공간에 카드 숫자 저장
        scanf("%d", &v[i]);
    }

    vector<int> three(3); //뽑는 세개의 카드
    int comparison = 0; //비교하는 변수
    int card_three_sum = 0; //카드의 합
    
    Combination(v, three, 3, 0, 0, max_number, comparison, card_three_sum);  // 조합 구하기
    cout << comparison; //최종 합 출력
}
