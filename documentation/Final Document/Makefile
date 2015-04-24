report.pdf: report.tex
	pdflatex report.tex
	bibtex report
	pdflatex report.tex
	pdflatex report.tex
# If there is a problem with the bibliography, just strip the
# \begin{thebibliography} and \end{thebibliography} lines from the
# aux file.

edit:
	vim report.tex

clean:
	rm report.pdf

