//
// Created by lunar on 02/12/17.
//


#include <algorithm>
#include <fstream>
#include <ios>
#include <iostream>
#include <map>
#include <string>
#include <sstream>
#include <vector>

class Program {

	std::string _name;

	std::vector<Program*> _children;
	std::vector<std::string> _childStrings;
	Program* _parent = nullptr;

	int _weight;

public:
	Program(const std::string& line) {

		std::stringstream ss(line);
		std::string tmp;

		std::getline(ss, _name, ' ');
		std::getline(ss, tmp, '(');
		std::getline(ss, tmp, ')');

		_weight = std::stoi(tmp);

		std::getline(ss, tmp, '>');

		if(!ss.eof()) {

			while (std::getline(ss, tmp, ',')) {
				tmp.erase(tmp.begin());
				_childStrings.push_back(tmp);

			}

		}

	}

	void organiseTreeStructure(std::vector<Program*>& programs) {

		for(const auto& n : _childStrings) {

			Program* program = *std::find_if(programs.begin(), programs.end(), [&](Program* pr) -> bool{
				return pr->getName() == n;
			});
			program->setParent(this);
			_children.push_back(program);

		}

	}

	std::string getName() { return _name; }

	void setParent(Program* parent) { _parent = parent; }
	Program* getParent() { return _parent; }

	int getWeight() { return _weight; }
	int getTowerWeigth() {
		int weight = getWeight();
		for(auto& c : _children) {
			weight += c->getTowerWeigth();
		}
		return weight;
	}

	int solvePart2() {

		return solvePart2(0);

	}

private:
	int solvePart2(int desiredWeight) {

		int corrWeight;
		std::map<int, int> weights;

		for(auto p : _children)
			weights[p->getTowerWeigth()]++;

		int highestWeight = 0;
		for(auto p : weights)
			if (p.second > highestWeight)
				highestWeight = p.first;
		int mostCommonWeight = highestWeight;

		auto incorrectProgram = std::find_if(_children.begin(), _children.end(), [&](Program* pr) -> bool{
			return pr->getTowerWeigth() != mostCommonWeight;
		});

		if(incorrectProgram == _children.end())
			return desiredWeight - (mostCommonWeight * _children.size());

		return (*incorrectProgram)->solvePart2(mostCommonWeight);

	}

};

extern "C" {
    int lunar_cpp(char* text, int len) {
        std::stringstream inFile(std::string(text, len));
        std::string line;

        std::vector<Program*> programs;

        while(std::getline(inFile, line))
            programs.push_back(new Program(line));

        for(auto p : programs)
            p->organiseTreeStructure(programs);

        Program* startNode = programs[0];

        while (startNode->getParent())
            startNode = startNode->getParent();

        return startNode->solvePart2();
    }
}
