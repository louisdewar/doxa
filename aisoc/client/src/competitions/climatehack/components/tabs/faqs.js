
export default [
  {
    group: 'Competition queries',
    questions: [
      {
        question: 'When do submissions for the first round close?',
        response: <>
          <p>
            Submissions close at 23:59 GMT on Wednesday 16th March.
          </p>
        </>
      },
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
        question: <>Training directly off the Zarr dataset seems to be fairly slow. Is there anything I can do?</>,
        response: <>
          <p>
            Unfortunately, it seems that indexing directly into the EUMETSAT Zarr dataset can be glacially slow due to the dask arrays used internally. Before you start training any models, you may want to consider preprocessing the dataset into ~5GB chunks of daylight satellite imagery (potentially taken only over Great Britain) stored in a much faster format, such as NumPy <code>.npz</code> (multi-array) or <code>.npy</code> (single-array) binary files.
          </p>
          <p>
            You may wish to perform additional steps, such as clipping the data to be definitively in the range <code>[0.0, 1023.0]</code>, at this stage.
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
            This is usually a scaling issue: the satellite image pixel values fall in the range <code>[0.0, 1023.0]</code>, but you may be returning values in the range <code>[0.0, 1.0]</code> or <code>[0.0, 255.0]</code> as is more common, which would artificially diminish your score. The images on DOXA look fine because they are normalised before being rendered to show contrasts better.
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
      {
        question: <>I cannot install pytorch-msssim to use inside a Jupyter notebook using conda. Is there a fix?</>,
        response: <>
          <p>
            If you cannot install it using <code>conda</code> directly, try to install it using <code>pip</code>, e.g.
          </p>
          <pre>
            pip install pytorch-msssim
          </pre>
          <p>
            Failing that, you may wish to try directly putting the installation command inside the Jupyter notebook, such as by adding the following cell at the top of your notebook.
          </p>
          <pre>
            !pre install pytorch-msssim
          </pre>
        </>,
      },
    ]
  }
];
