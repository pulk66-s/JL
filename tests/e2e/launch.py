import os

def get_all_files():
    files_list = []
    for root, dirs, files in os.walk("."):
        for file in files:
            if file.endswith(".cl"):
                files_list.append(os.path.join(root, file))
    return files_list

def launch_test(file):
    os.system(f"./../../jl {file} > {file}.out")
    status = os.system(f"diff {file}.out {file}.expected")
    return os.WEXITSTATUS(status)

def launch_tests(files_list):
    success, failure = 0, 0
    for file in files_list:
        res = launch_test(file)
        if res != 0:
            print(f"Test {file} failed")
            failure += 1
        else:
            print(f"Test {file} passed")
            success += 1
    return success, failure

def delete_all_output_files(files_list):
    for file in files_list:
        os.system(f"rm {file}.out")

def change_dir():
    current_program_dir = os.path.dirname(os.path.realpath(__file__))
    os.chdir(current_program_dir)

def main():
    change_dir()
    files_list = get_all_files()
    success, failure = launch_tests(files_list)
    delete_all_output_files(files_list)
    print(f"Success: {success}, Failure: {failure}")
    exit(failure)

if __name__ == "__main__":
    main()
