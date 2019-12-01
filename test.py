def main():
    try:
        print(0/0)
    except Exception as e:
        print('\033[91m' + str(e) + '\033[0m')


main()
