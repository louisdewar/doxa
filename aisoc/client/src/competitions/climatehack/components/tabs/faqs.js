
export default [
  {
    group: 'Competition queries',
    questions: [
      {
        question: 'Can we participate in teams?',
        response: <>
          <p>
            Although you compete in the first round as an individual, we would encourage you to collaborate, share ideas, and discuss strategies and approaches with each other! It may be a competition, but we are united in the fight against climate change.
          </p>
          <p>
            Climate Hack.AI is also an excellent opportunity to get to know people, not just at your university, but also at world-leading universities from across the world, so definitely make the most of the intra-competition events we will be holding.
          </p>
        </>
      },
      {
        question: 'Does winning the first round mean I will win the finals?',
        response: <>
          <p>
            In the finals, submissions will be graded against an expanded test set with completely unseen data, so already their rankings may vary. Furthermore, judges will be interested in more than just your MS-SSIM score; they will also be looking to evaluate your contribution to advancing the state of the art and the quality of your submission&apos;s output in other ways. It is all to play for!
          </p>
        </>
      }
    ]
  },
  {
    group: 'Submission-related issues',
    questions: [
      {
        question: 'How many submissions can I make a day?',
        response: <>
          <p>
            You can make up to eight submissions a day, four an hour and two every five minutes.
          </p>
          <p>
            Currently, submissions are limited to 4GB in size; however, this limit will be increased as the competition progresses.
          </p>
          <p>
            The virtual machine environment in which submissions are evaluated has 4 vCPUs and 6GB RAM.
          </p>
        </>,
      },
      {
        question: <>Can my model&apos;s leaderboard score change?</>,
        response: <>
          <p>
            The dataset used to test submissions in the evaluation environment is periodically refreshed, so it is possible that your model&apos;s score may change slightly over time. The complete test set used to determine participation in the finals will be in place closer to the end of the competition, so for now, just focus on training the best model you can that performs well on unseen imagery!
          </p>
        </>,
      },
      {
        question: <>I don&apos;t see my agent on the leaderboard after uploading it. What should I do?</>,
        response: <>
          <p>
            If this happens, there was most likely an error processing your submission. If you are logged in on DOXA, you can view the <code>stderr</code> output of your agent by clicking &lsquo;Your submission&rsquo; in the leaderboard tab. Alternatively, you can view your latest submission from the account page.
          </p>
        </>,
      },
      {
        question: <>How can I fix a &lsquo;wrong output type&rsquo; error?</>,
        response: <>
          <p>
            DOXA expects your code to produce NumPy arrays of type <code>numpy.float32</code>. If you return an array of type <code>numpy.float64</code> or anything else, DOXA cannot grade your submission.
          </p>
          <p>
            You can usually fix this by casting your output NumPy array to the right type, e.g. by running something equivalent to <code>output.astype(numpy.float32)</code>. Recall that your model should output pixel values in the range 0.0 to 1023.0 (inclusive).
          </p>
        </>,
      },
      {
        question: <>My model outputs images that look fine, but my score is low. What&apos;s wrong?</>,
        response: <>
          <p>
            This is usually a scaling issue: the satellite image pixel values fall in the range <code>[0.0, 1023.0]</code>, but you may be returning values in the range <code>[0.0, 1.0]</code> or <code>[0.0, 255.0]</code> as is more common, which would artificially diminish your score.
          </p>
          <p>
            If you are certain that it is not, do reach out to us on Discord!
          </p>
        </>,
      },
      {
        question: <>My agent crashes with &lsquo;Attempting to deserialize object on a CUDA device but torch.cuda.is_available() is False&rsquo;. What can I do?</>,
        response: <>
          <p>
            The evaluation environment does not have a CUDA-enabled GPU, so attempting to deserialise the model with <code>load_state_dict</code> will fail. To remedy this, you can replace your <code>torch.load(&apos;model.pt&apos;)</code> call (or equivalent) with the following:
          </p>
          <pre>
            torch.load(&apos;model.pt&apos;, map_location=torch.device(&apos;cpu&apos;))
          </pre>
        </>,
      },
    ]
  }
];
