import csv

with (
    open("word_list.txt", "w") as word_file,
    open("word_freq.txt", "w") as freq_file,
    open("word_frequency.csv", "r") as data_file,
):
    reader = csv.DictReader(data_file)
    for row in reader:
        word_file.writelines(row["lemma"] + "\n")
        freq_file.writelines(row["freq"] + "\n")
