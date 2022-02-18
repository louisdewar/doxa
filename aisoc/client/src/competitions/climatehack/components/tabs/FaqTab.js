import { useEffect, useState } from 'react';
import faqs from './faqs';
import './FaqTab.scss';


function FaqGroup({ questions }) {
  const [open, setOpen] = useState({});

  useEffect(() => {
    const initial = {};
    for (const i in questions) {
      initial[i] = false;
    }
    setOpen(initial);
  }, [questions]);

  const toggleOpen = question => {
    setOpen({
      ...open,
      [question]: !open[question],
    });
  };

  return questions.map((question, i) => <div key={i}>
    <h4>{open[i] ? <>&#x25BC;</> : <>&rsaquo;</>} <a onClick={() => toggleOpen(i)}>{question.question}</a></h4>
    {open[i] && question.response}
  </div>);
}


export default function FaqTab() {


  return <div className="ch-tab ch-faq-tab">
    <h2>Frequently Asked Questions</h2>
    <p>
      If you have a question that has not been answered here, ask us on <a href="https://discord.gg/HTTQ8AFjJp">Discord</a>, where we have several help and FAQ channels!
    </p>
    <h3>{faqs[0].group}</h3>
    <FaqGroup questions={faqs[0].questions} />

    <h3>{faqs[1].group}</h3>
    <FaqGroup questions={faqs[1].questions} />
  </div>;
}
