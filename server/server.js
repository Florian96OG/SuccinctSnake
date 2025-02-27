const express = require('express');
const { execFile } = require('child_process');
const app = express();
app.use(express.json());

app.post('/prove', (req, res) => {
    const events = req.body; // Array of { type, timestamp }
    const eventsJson = JSON.stringify(events);
    
    // Path to your ELF (adjust as needed)
    execFile('../score_proof/target/release/score_proof', [eventsJson], (err, stdout, stderr) => {
        if (err) {
            console.error("ELF error:", stderr);
            return res.status(500).json({ error: stderr });
        }
        // Parse ELF output (assumes "Score: X, Proof: [bytes]")
        const [scorePart, proofPart] = stdout.split(', Proof: ');
        const score = parseInt(scorePart.replace("Score: ", ""));
        const proof = JSON.parse(proofPart); // Adjust if output differs
        res.json({ score, proof });
    });
});

app.listen(3000, () => console.log('Proof server running on port 3000'));
