function bindQuizzes() {
  document.querySelectorAll(".quiz:not([data-quiz-bound])").forEach((quiz) => {
    quiz.dataset.quizBound = "true";
    quiz.querySelector(".quiz-check").addEventListener("click", () => {
      const answer = quiz.querySelector("input:checked");
      const result = quiz.querySelector(".quiz-result");
      quiz.querySelectorAll(".quiz-correct").forEach((label) => {
        label.classList.remove("quiz-correct");
      });
      if (!answer) {
        result.textContent = "Choose an answer first.";
        return;
      }
      if (answer.value === quiz.dataset.answer) {
        answer.closest("label").classList.add("quiz-correct");
        result.textContent = "✓ Correct.";
        if (quiz.dataset.explanation) {
          result.append(document.createElement("br"));
          result.append(document.createTextNode(quiz.dataset.explanation));
        }
      } else {
        result.textContent = "Try again.";
      }
    });
  });
}

if (typeof document$ === "undefined") bindQuizzes();
else document$.subscribe(bindQuizzes);
