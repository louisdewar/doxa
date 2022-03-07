import Card from 'components/Card';
import Container from 'components/Container';
import Footer from 'components/Footer';
import Navbar from 'components/Navbar';


export default function Rules() {
  return <div className='main-wrapper'>
    <Navbar />
    <Container>
      <Card>
        <h1>Competition Rules</h1>
        <div style={{ whiteSpace: 'pre-wrap' }}>
          <p>
            {'RULES: ENTRY IN THIS COMPETITION CONSTITUTES YOUR ACCEPTANCE OF THESE OFFICIAL COMPETITION RULES.'}
          </p>
          <p>
            {'The Competition is a skills-based competition to promote and further the field of data science and machine learning. You must register via the Competition Website to enter. Your Submissions must conform to the requirements stated on the Competition Website. Your Submissions will be scored based on the evaluation metrics described on the Competition Website and in accordance with Section 10 below.'}
          </p>
          <p>
            {'Subject to compliance with the Rules, Prizes, if any, will be awarded to participants with the best scores, based on the merits of the machine learning models submitted.'}
          </p>

          <h2>1.	BINDING AGREEMENT.</h2>
          <p>{'1.1.	To enter the Competition, you must agree to these Rules, which incorporate by reference the provisions and content of the Competition Website. Please read these Rules carefully before entry to ensure you understand and accept them. You further agree that submission of an entry in the Competition constitutes agreement to these Rules. You may not submit an entry to the Competition and are not eligible to receive the Prizes unless you agree to these Rules. These Rules form a binding legal agreement between you and the Competition Organiser with respect to the Competition.'}</p>

          <h2>2.	ELIGIBILITY.</h2>
          <p>{`2.1.	To be eligible to compete in Climate Hack.AI, you must be over 18 years old and currently enrolled as an undergraduate, master’s or doctoral student at one of the Participating Universities.
2.2.	If you are entering as a representative of a Co-Hosting Society, these rules are binding on you, individually, and the society that you represent. You warrant that entry into the Competition does not violate any of your university’s policies.
2.3.	The Competition Organiser reserves the right to verify eligibility and to adjudicate on any dispute at any time. If you provide any false information relating to the Competition concerning your identity, residency or any other information that relates to your entry into the Competition, you may be immediately disqualified from the Competition.
2.4.	Teams from Competition Organiser may enter the Competition provided that they do not include an Excluded Participant. If you are a participant from the Competition Organiser, you are subject to all applicable internal policies of the Competition Organiser with respect to your participation.`}</p>

          <h2>3.	INDIVIDUALS AND TEAMS.</h2>
          <p>{`3.1.	Individual Account. You may make Submissions only under one, unique climatehack.ai account. You will be disqualified if you make Submissions through more than one account, or attempt to falsify an account to act as your proxy. You may submit up to the maximum number of Submissions per day as specified on the Competition Website.
3.2.	Teams. Team membership for each Participating University for attending the Competition Finals will be determined strictly by each Participant’s position on the leaderboard at the conclusion of the Qualifying Rounds. The three highest ranking individuals from each Participating University shall form the team that will attend the Competition Final. In the event that one or more members of the same Participating University have the same score, then the Participant that made his submission first will rank higher than the Participant that submitted at a later time.`}</p>

          <h2>4.	ENTRY TO THE COMPETITION.</h2>
          <p>{`4.1.	To enter the Competition, you must register on the Competition Website prior to the Entry Deadline, and follow the instructions for developing and entering your Submission through the Competition Website.
4.2.	Your Submissions must be made in the manner and format, and in compliance with all other requirements, stated on the Competition Website (the "Requirements").
4.3.	Submissions must be received before any Submission deadlines stated on the Competition Website. Submissions not received by the stated deadlines will not be eligible to receive a Prize.
4.4.	Submissions are void if they are in whole or part illegible, incomplete, damaged, altered, counterfeit, obtained through fraud, or late. Competition Organiser reserves the right to disqualify any entrant who does not follow these Rules, including making a Submission that does not meet the Requirements.`}</p>

          <h2>5.	DISQUALIFICATION & FORFEIT OF PRIZE.</h2>
          <p>{`5.1.	Competition Organiser reserves the right to disqualify any participant (either individually or as a Team) from the Competition if the Competition Organiser, acting reasonably, believes that the participant has cheated in any way, submits malevolent code or abuses, threatens or harasses any other participants or any Competition Entities.
5.2.	At the Competition Organiser's sole discretion, a disqualified participant may be removed from the Competition leaderboard and forfeit any claim to the Competition Prize.
5.3.	Determinations of Competition Organiser are final and binding`}</p>
          <h2>6.	COMPETITION TIMELINE.</h2>
          <p>{`6.1.	The Competition Timeline is subject to change, and the Competition Organiser may introduce additional hurdle deadlines during the Competition Period. Any updated or additional deadlines will be publicized on the Competition Website.
6.2.	It is the participant’s responsibility to check the Competition Website regularly to stay informed of any deadline changes.`}</p>

          <h2>7.	SPONSORS AND DATA PROVIDER.</h2>
          <p>{`7.1.	The Competition is sponsored by the Competition Sponsors whom are not part of any agreement with you except as specifically provided in these Rules. You understand that the Competition Sponsors have no responsibility with respect to selecting the potential Competition winner(s) or awarding any Prizes.
7.2.	The data set for the Competition is provided by the Data Provider whom are not part of any agreement with you except as specifically provided in these Rules.`}</p>

          <h2>8.	COMPETITION DATA.</h2>
          <p>{`8.1.	You may access and use the Competition Data only for participating in the Competition and on climatehack.ai forums. The Competition Organiser reserves the right to disqualify any participant who uses the Competition Data other than as permitted by the Competition Website and these Rules.
8.2.	You agree to keep the Competition Data confidential not to transmit, duplicate, publish, or redistribute the Competition Data to any party that is not participating in the Competition and use reasonable and suitable measures to prevent persons who have not formally agreed to these Rules from gaining access to the Competition Data. In the event that you become aware of any data leaks, you agree to notify the Data Provider immediately and agree to work with them to rectify any unauthorized transmission or access.
8.3.	You may use data other than the Competition Data (“External Data”) to develop and test your Submissions. However, you will ensure the External Data is publicly available and equally accessible to use by all participants of the Competition for purposes of the competition at no cost to the other participants.
8.4.	The Competition Data will contain private and public test sets. Which data belongs to which set will not be made available to participants.`}</p>

          <h2>9.	 REQUIREMENTS FOR SUBMISSION OF CODE.</h2>
          <p>{`9.1.	If open-source code is used in the model to generate the Submission, then you must only use open-source code licensed under an Open-Source Initiative-approved license that does not limit the commercial use of such code or model containing or depending on such code.
9.2.	Sharing source or executable code developed in connection with or based upon the Competition Data or other source or executable code relevant to the Competition (“Competition Code”) can be shared provided that such public sharing does not violate the rights of any third party. By so sharing, you are deemed to have licensed the shared code under an Open-Source Initiative-approved license that does not limit the commercial use of such Competition Code or model containing or depending on such Competition Code.
9.3.	Individual participants and Teams may use automated machine learning tools (“AMLT”) (e.g., Google AutoML, H2O Driverless AI, etc.) to create a Submission, provided that the participant or Team ensures that they have an appropriate license to the AMLT such that they are able to comply with the Rules.`}</p>

          <h2>10.	SCORING OF SUBMISSIONS.</h2>
          <p>{`10.1.	Each Submission will be scored and ranked by testing the predictions against a private test set using a scoring function called MultiScale Structural Similarity (MS_SSIM) and any other methods that are deemed relevant by the Judges.
10.2.	During the Competition Period, the current ranking will be visible on the Competition Website's public leaderboard. The potential winner(s) are determined by the leaderboard ranking on the private leaderboard, subject to compliance with these Rules and any other methods that are deemed relevant by the Judges. The public leaderboard will be based on the public test set and the private leaderboard will be based on the private test set.
10.3.	In the event of a tie, the respective prizes will be split equally between the winning parties. For example, if two Teams tied for first place in the Competition, the prizes for first and second place would be added together and divided equally between the two teams. The next highest scoring team would be awarded the prize for third place.
10.4.	In the event a potential winner is disqualified for any reason, the Submission that received the next highest score rank will be chosen as the potential winner.`}</p>

          <h2>11.	NOTIFICATION OF WINNERS.</h2>
          <p>{'11.1.	The potential winner(s) will be notified as soon as possible after the final event has been concluded.'}</p>

          <h2>12.	WINNERS OBLIGATIONS.</h2>
          <p>{`12.1.	As a condition to being awarded a Prize, a Prize winner must fulfil the following obligations:
12.1.1.	deliver to the Data Provider the final model's software code as used to generate the winning Submission and associated documentation. The delivered software code must be capable of generating the winning Submission, and contain a description of resources required to build and/or run the executable code successfully. To the extent that the final model’s software code includes generally commercially available software that is not owned by you, but that can be procured by the Data Provider without undue expense, then instead of delivering the code for that software to the Data Provider, you must identify that software, method for procuring it, and any parameters or other information necessary to replicate the winning Submission;
12.1.2.	grant to Data Provider and its designees a worldwide, non-exclusive, sub-licensable (through multiple tiers), transferable, fully paid-up, royalty-free, perpetual, irrevocable right to use, reproduce, distribute, create derivative works of, publicly perform, publicly display, digitally perform, make, have made, sell, offer for sale and import your winning Submission and the source code used to generate the Submission, in any media now known or developed in the future, for any purpose whatsoever, commercial or otherwise, without further approval by or payment to you. To the extent your Submission makes use of generally commercially available software not owned by you that you used to generate your Submission, but that can be procured by the Data Provider without undue expense, you do not grant the license in the preceding sentence to that software. You will also represent that you have the unrestricted right to grant that license;
12.1.3.	sign and return all Prize acceptance documents as may be required by Competition Organiser including without limitation: (i) eligibility certifications; (ii) licenses, releases and other agreements required under the Rules; and (iii) any applicable tax forms`}</p>

          <h2>13.	PRIZES.</h2>
          <p>{`13.1.	Prizes are set out in the defined terms and are awarded on the basis of both a Team’s final position on the Leaderboard and the value judgements of the expert Judges. The Judges’ decision will be final and no appeals will be permitted.
13.2.	All Prizes are subject to Competition Organiser’s review and verification of the participant’s eligibility and compliance with these Rules, and the compliance of the winning Submissions with the Requirements.
13.3.	In the event that the Submission demonstrates non-compliance with Rules, Competition Organiser may at its sole discretion either disqualify the Submission(s) or require the potential winner to remediate all issues identified in the Submission(s) within five days after written notice.
13.4.	Potential winners must return all required Prize acceptance documents within two (2) weeks following notification of such required documents, or such potential winner will be deemed to have forfeited the prize and another potential winner will be selected.
13.5.	Prize(s) will be awarded by way of bank transfer to the winner’s nominated bank account within approximately 30 days after receipt by Competition Organiser of the required Prize acceptance documents.
13.6.	The Prize money will be allocated in even shares between the eligible Team members.
13.7.	It is the Prize winners’ responsibility to ensure that the correct bank details are submitted to the Competition Organiser.
`}</p>

          <h2>14.	TAXATION.</h2>
          <p>{`14.1.	The winners are responsible for the payment of all taxes that are due on the Prizes.
14.2.	Payments to potential winners are subject to the express requirement that they submit all documentation requested by Competition Organiser for compliance with applicable tax reporting and withholding requirements. If a potential winner fails to provide any required documentation or comply with applicable laws, the Prize may be forfeited and Competition Organiser may select the participant’s society as an alternative potential winner.
14.3.	Prizes will be net of any taxes that Competition Organiser is required by law to withhold.`}</p>

          <h2>15.	PUBLICITY.</h2>
          <p>{'15.1.	You agree that Competition Organiser, the Competition Sponsors and their affiliates may use your or your Team’s name and likeness for advertising and promotional purposes without additional compensation, unless prohibited by law.'}</p>

          <h2>16.	PRIVACY.</h2>
          <p>{`16.1.	You acknowledge and agree that Competition Organiser may collect, store, share and otherwise use personally identifiable information provided by you during the registration process and the Competition, including but not limited to, name, mailing address, phone number, and email address (“Personal Information”).
16.2.	The Competition Organiser acts as an independent controller with regard to its collection, storage, sharing, and other use of this Personal Information, and will use this Personal Information in accordance with its Privacy Policy https://www.ucl.ac.uk/legal-services/privacy/general-privacy-notice, including for administering the Competition.
16.3.	If you are a Competition winner, you acknowledge and agree that the Competition Organiser may transfer to Competition Sponsor your name, email address, and country information, but only to the extent that you provide Competition Organiser  with such information, and  any applicable data transfer laws permit such transfer of your information; and  if such information is provided to Competition Sponsor, Competition Sponsor may use such information only in accordance with the UCL Privacy Policy Notice https://www.ucl.ac.uk/legal-services/privacy/general-privacy-notice.`}</p>

          <h2>17.	WARRANTY, INDEMNITY AND RELEASE.</h2>
          <p>{`17.1.	You warrant that your Submission is your own original work and that you are the sole and exclusive owner and rights holder of the Submission, and you have the right to make the Submission and grant all required licenses.
17.2.	You agree not to make any Submission that: infringes any third party rights or otherwise violates any applicable law.
17.3.	To the maximum extent permitted by law, you indemnify and agree to keep indemnified Competition Entities at all times from and against any liability, claims, demands, losses, damages, costs and expenses resulting from any of your acts, defaults or omissions and/or a breach of any warranty set forth herein.
17.4.	To the maximum extent permitted by law, you agree to defend, indemnify and hold harmless the Competition Entities from and against any and all claims, actions, suits or proceedings, as well as any and all losses, liabilities, damages, costs and expenses (including reasonable legal fees) arising out of or accruing from your Submission or other material uploaded or otherwise provided by you that infringes any third party rights or your participation in the Competition and any Competition-related activity.
17.5.	You hereby release Competition Entities from any liability associated with any problem with the Competition Website, any error in the collection, processing, or retention of any Submission or any typographical or other error in the printing, offering or announcement of any Prize or winners.
`}</p>

          <h2>18.	FORCE MAJEURE.</h2>
          <p>{`18.1.	Competition Entities are not responsible for any malfunction of the Competition Website or any hardware or software failures of any kind whatsoever which limit a participant’s ability to participate.
18.2.	Competition Entities are not responsible for any postponement or cancellation of the Competition for reasons outside of their control such as COVID-related travel restrictions that might be imposed during the Competition Timeline`}</p>

          <h2>19.	TRAVEL.</h2>
          <p>{`19.1.	 Any participant who must travel from abroad for the in-person finals is responsible for acquiring the necessary documentation and visas to enter the UK or the USA (depending on which they are traveling to).
19.2.	Any participant who must travel from abroad for the in-person finals is responsible for meeting whatever other travel requirements are prevailing at the time (e.g. COVID tests or vaccination status).
19.3.	 It is up to the participant to ensure that they understand the travel requirements fully and submit whatever documentation is required. Under no circumstances does the Competition Organiser accept any responsibility for any flights that are cancelled due to the participant failing to provide the airline with the required documentation on a timely basis.`}</p>

          <h2>20.	RIGHT TO CANCEL OR MODIFY.</h2>
          <p>{'20.1.	If for any reason the Competition is not capable of running as planned for whatever reason, the Competition Organiser reserves the right to cancel, terminate, modify or suspend the Competition in its sole discretion.'}</p>

          <h2>21.	GOVERNING LAW.</h2>
          <p>{'21.1.	This Agreement shall be governed by and construed in accordance with the laws of England and Wales and the parties irrevocably submit to the exclusive jurisdiction of the English courts for all claims arising out of or relating to these Rules.'}</p>

          <h2>22.	SEVERABILITY.</h2>
          <p>{'22.1.	The invalidity or unenforceability of any provision of these Rules shall not affect the validity or enforceability of any other provision of the Rules.'}</p>

          <h2>DEFINITIONS:</h2>
          <ul>
            <li>{`Co-Hosting Societies
Shall mean Caltech Data Science Organization, Data Science Club at Carnegie Mellon, Columbia Data Project Initiative, Cornell Data Science, Big Data Big Impact, Harvard Machine Intelligence Community, Imperial College Data Science Society, AI Club at MIT, Princeton Data Science, CS+Social Good, UCL Artificial Intelligence Society, Bristol Computer Science Society, Machine Learning @ Berkeley, UCLA ACM AI, Cambridge University Artificial Intelligence, Edintelligence, Glasgow University Tech Society, ACM Artificial Intelligence & Data Analytics (AIDA) SIG, MUDSS Data Science Society, Michigan Data Science Team, Oxford Artificial Intelligence Society, SaIntelligence, UofT AI, University of Toronto Machine Intelligence Student Team, Warwick AI, Waterloo Data Science Club.`}</li>
            <li>{`Competition
Shall mean Climate Hack.AI`}</li>
            <li>{`Competition Data
Shall mean the data or datasets provided by the Data Provider and available from the Competition Website for the purpose of use in the Competition, including any prototype or executable code provided on the Competition Website`}</li>
            <li>{`Competition Entities
Shall mean the Competition Organiser, the Competition Sponsors, the Data Provider and their respective parent companies, subsidiaries and affiliates`}</li>
            <li>{`Competition Finals
Shall mean the finals of the Competition that will be held on 24 to 26 March 2022 in London for Participating Universities based in the United Kingdom and Boston for Participating Universities based in the United States of America or other such date as communicated by the Competition Organiser from time to time.`}</li>
            <li>{`Competition Organiser
Shall mean University College London Artificial Intelligence Society`}</li>
            <li>{`Competition Period
The period from the Start Date to the Final Submission Deadline as set out on the Competition Website from time to time`}</li>
            <li>{`Competition Sponsors
Ennovate and Newcross Healthcare Solutions Limited`}</li>
            <li>{`Competition Timeline
Competition Timeline dates (including Entry Deadline, Start Date, Final Submission Deadline and Competition Finals, as applicable) are contained on Competition Website`}</li>
            <li>{`Competition Website
https://climatehack.ai`}</li>
            <li>{`Date Provider
Open Climate Fix (using EUMETSAT data)`}</li>
            <li>{`Excluded Participant
Any Development Officer, the Head of Development or President of the Competition Organiser.`}</li>
            <li>{`Judges
The panel of experts as selected by the Competition Organiser from time to time at its sole discretion to evaluate the Submissions`}</li>
            <li>{`Number of Accounts per Participant
1 (one). Participants are strictly prohibited from multiple entries including holding multiple accounts as set out in clause 3.1.`}</li>
            <li>{`Participating Universities
California Institute of Technology, Carnegie Mellon University, Columbia University, Cornell University, Georgia Institute of Technology, Harvard University, Imperial College London, Massachusetts Institute of Technology, Princeton University, Stanford University, University College London, University of Bristol, University of California Berkeley, University of California Los Angeles, University of Cambridge, University of Edinburgh, University of Glasgow, University of Illinois at Urbana-Champaign, University of Manchester, University of Michigan, University of Oxford, University of St. Andrews, University of Toronto, University of Warwick, University of Waterloo.`}</li>
            <li>
              Prizes
              <ul>
                <li>{`Leaderboard Prizes (Team):
First Prize: £30,000
Second Prize: £9,000
Third Prize: £6,750`}</li>
                <li>{`Club/Society Prizes:
First Prize: £2,500
Second Prize: £1,000
Third Prize: £750`}</li>
                <li>{'Total Prize Pool: £50,000'}</li>
              </ul>
            </li>
            <li>{`Qualifying Rounds
The rounds of the Competition that determine the Team membership for the final rounds of the Competition.`}</li>
            <li>{`Submissions
Submissions to the Competition`}</li>
            <li>{`Submission Limits
Up to 8 entries per day may be submitted. Entries in excess of this number will be void.`}</li>
            <li>{`Team
A group of 3 participants from a Co-hosting Society.`}</li>
          </ul>
        </div>
      </Card>
    </Container>
    <Footer />
  </div >;
}
