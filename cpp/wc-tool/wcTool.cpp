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
            lines++;
            chars += line.size();
            words += word_count(line);
            bytes += sizeof(line);
        }
        myfile.close();
    } else cout << "Unable to open file" << endl;
    
    // initialize FileData with variable data
    FileData file_data(bytes, lines, chars, words);

    // match query param
    if (config.query_param == "-l") {
        cout << file_data.lines << " " << config.file_path << endl;
    } else if (config.query_param == "-c") {
        cout << file_data.bytes << " " << config.file_path << endl;
    } else if (config.query_param == "-w") {
        cout << file_data.words << " " << config.file_path << endl;
    } else if (config.query_param == "-m") {
        cout << file_data.chars << " " << config.file_path << endl;
    }
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