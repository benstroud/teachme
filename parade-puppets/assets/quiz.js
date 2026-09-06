/**
 * Parade Puppets — Quiz Widget
 *
 * Usage: Add a .quiz-section with .quiz-question elements.
 * Each question needs data-correct="0" (zero-indexed) on the container,
 * and .quiz-option buttons inside.
 *
 * <div class="quiz-section">
 *   <div class="quiz-question" data-correct="2">
 *     <p class="quiz-prompt">What is the thing?</p>
 *     <button class="quiz-option">Option A</button>
 *     <button class="quiz-option">Option B</button>
 *     <button class="quiz-option">Option C</button>
 *     <p class="quiz-explanation">Explanation shown after answering.</p>
 *   </div>
 * </div>
 */

document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('.quiz-question').forEach(q => {
    const correctIdx = parseInt(q.dataset.correct, 10);
    const options = q.querySelectorAll('.quiz-option');
    const explanation = q.querySelector('.quiz-explanation');
    if (explanation) explanation.style.display = 'none';

    options.forEach((btn, idx) => {
      btn.addEventListener('click', () => {
        // Prevent re-answering
        if (q.classList.contains('answered')) return;
        q.classList.add('answered');

        options.forEach((b, i) => {
          b.disabled = true;
          if (i === correctIdx) {
            b.classList.add('correct');
          } else {
            b.classList.add('incorrect');
          }
        });

        if (idx === correctIdx) {
          btn.classList.add('selected-correct');
        } else {
          btn.classList.add('selected-incorrect');
        }

        if (explanation) {
          explanation.style.display = 'block';
        }
      });
    });
  });
});
