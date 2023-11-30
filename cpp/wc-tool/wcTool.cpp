#include <iostream>
#include <fstream>
#include <iostream>
#include <string>
#include <sstream>

using namespace std;

class Config {
    public:
        std::string file_path;
        std::string query_param;

    void parse_config(char *argv[]) {
        if (argv[1] != nullptr && argv[2] != nullptr) {
            file_path = argv[1];
            query_param = argv[2];

            std::cout << file_path << endl;
            std::cout << query_param << endl;
        } else {
            std::cerr << "Error: Insufficient command-line arguments.\n";
            std::exit(EXIT_FAILURE);
        }
    }
};

class FileData {
    public:
        int bytes;
        int lines;
        int chars;
        int words;

    FileData(int b, int l, int c, int w) : bytes(b), lines(l), chars(c), words(w) {}
};

int word_count(std::string const& str) {
    std::stringstream  stream(str);
    std::string        oneWord;
    unsigned int       count = 0;

    while(stream >> oneWord) { ++count;};

    return count;
}

void run(Config config) {
    // initialize variables
    int bytes = 0;
    int lines = 0;
    size_t chars = 0;
    int words = 0;

    // read file and calculate variables
    ifstream myfile (config.file_path);
    std::string line;
    if (myfile.is_open()) {
        while(getline(myfile, line)) {
            cout << line << endl;
            lines++;
            chars += line.size();
            words += word_count(line);
        }
        myfile.close();
    } else cout << "Unable to open file" << endl;

    cout << "Lines: " << lines << endl;
    cout << "Characters: " << chars << endl;
    cout << "Words: " << words << endl;
    // initialize FileData with variable data
    // match query param
}



int main(int argc, char *argv[]) {
    if (argc < 3) {
        std::cerr << "Usage: " << argv[0] << " <file_path> <query_param>\n";
        return EXIT_FAILURE;
    }

    Config config;
    config.parse_config(argv);

    run(config);

    return 0;
}