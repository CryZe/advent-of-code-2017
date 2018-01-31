#include <iostream>
#include <limits>
#include <stdio.h>
#include <stdlib.h>
#include <fstream>
#include <vector>
#include <map>
#include <set>
#include <algorithm>
#include <functional>
#include <cctype>
#include <locale>
#include <sstream>
#include <regex>
#include <tuple>

using namespace std;

typedef tuple<int, int, vector<string>> WeightsAndChildren;

int sumWeights(vector<string> &parents, vector<WeightsAndChildren> &nodes, int idx, int& part2) {
    vector<int> subWeights(get<2>(nodes[idx]).size());
    int i= 0;
    int sum= 0;
    for(string s: get<2>(nodes[idx])) {
        int subIdx= find(parents.begin(), parents.end(), s)-parents.begin();
        subWeights[i]= sumWeights(parents, nodes, subIdx, part2);
        sum+= subWeights[i];
        ++i;
    }
    if(subWeights.size()>0) {
        int i;
        for(i= 1; i<subWeights.size(); ++i) {
            if(subWeights[i]!=subWeights[0])
                break;
        }
        if(i<subWeights.size()) {
            int diff;
            if(i==1 && subWeights.size()>2 && subWeights[1]==subWeights[2]) {
                diff= subWeights[1]-subWeights[2];
                i= 0;
            } else {
                diff= subWeights[i]-subWeights[0];
            }
            string s= get<2>(nodes[idx])[i];
            int subIdx= find(parents.begin(), parents.end(), s)-parents.begin();
            // cout << "weight of \"" << parents[subIdx] << "\" is " << get<0>(nodes[subIdx]) << " should be " << get<0>(nodes[subIdx]) - diff << '\n';
            part2 = get<0>(nodes[subIdx]) - diff;
            sum-= diff;
        }
    }

    get<1>(nodes[idx])= get<0>(nodes[idx]) + sum;
    return get<1>(nodes[idx]);
}

extern "C" {
    int some_guy_cpp(char* text, int len) {
        stringstream infile(string(text, len));

        int part2 = 0;

        string line;
        vector<string> parents;
        vector<string> all_children;
        vector<WeightsAndChildren> nodes;
        while (getline(infile, line)) {
            static const regex re_arrow{"\\s*->\\s*"};
            vector<string>  line_split {
                sregex_token_iterator(line.begin(), line.end(), re_arrow, -1),
                sregex_token_iterator()
            };

            static const regex re_paren{"\\s+\\("};
            vector<string>  parent_split {
                sregex_token_iterator(line_split[0].begin(), line_split[0].end(), re_paren, -1),
                sregex_token_iterator()
            };

            parents.push_back(parent_split[0]);

            if(line_split.size()>1) {
                static const regex re_comma{"\\s*,\\s*"};
                vector<string>  children {
                    sregex_token_iterator(line_split[1].begin(), line_split[1].end(), re_comma, -1),
                    sregex_token_iterator()
                };
                for(auto s: children)
                    all_children.push_back(s);

                nodes.push_back(WeightsAndChildren(stoi(parent_split[1]), 0, children));
            } else {
                vector<string>  children;
                nodes.push_back(WeightsAndChildren(stoi(parent_split[1]), 0, children));
            }
        }

        string root;
        for(int i= 0; i<parents.size(); ++i) {
            if(find(all_children.begin(), all_children.end(), parents[i]) == all_children.end()) {
                root= parents[i];
                // cout << "root node is \"" << root << "\"" << '\n';
            }
        }
        int rootIdx= find(parents.begin(), parents.end(), root) - parents.begin();
        sumWeights(parents, nodes, rootIdx, part2);

        return part2;
    }
}
