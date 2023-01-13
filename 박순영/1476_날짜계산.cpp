#include<iostream>
#include<vector>
using namespace std;

int main() {
	int earth;
	int sun;
	int moon;
	cin >> earth >> sun >> moon;

	while (!((earth == sun) && (sun == moon))) {
		if (((earth <= sun) && (earth < moon))||((earth < sun) && (earth <= moon))) {
			earth = earth + 15;
		}
		else if (((sun <= earth) && (sun < moon))||((sun < earth) && (sun <= moon))) {
			sun = sun + 28;
		}
		else if (((moon <= sun) && (moon < earth)) || ((moon < sun) && (moon <= earth))) {
			moon = moon + 19;
		}

	}
	cout << earth;

}
