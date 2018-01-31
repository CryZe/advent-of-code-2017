#include <string_view>

extern "C" {
    struct Result {
        int part1;
        int part2;
    };

    Result kenira(char* txt, int len) {
        std::string_view inp(txt, len);

        bool garbage = false;
        bool ignore = false;
        int score = 0;          // total score
        int score_inc = 1;      // how much next group is worth
        int characters = 0;

        for (auto&& c : inp)
        {
            bool count = true;

            if (ignore == true)
            {
                ignore = false;
                count = false;
            }
            else if (c == '!' && garbage == true)
            {
                ignore = true;
                count = false;
            }
            else if (c == '<' && garbage == false)
            {
                garbage = true;
                count = false;
            }
            else if (c == '>' && garbage == true)
            {
                garbage = false;
                count = false;
            }
            else if (c == '{' && garbage == false)
            {
                score += score_inc;
                score_inc++;
            }
            else if (c == '}' && garbage == false)
            {
                score_inc--;
            }

            if (count == true && garbage == true)
            {
                characters++;
            }
        }

        Result ok;
        ok.part1 = score;
        ok.part2 = characters;
        return ok;
    }
}
