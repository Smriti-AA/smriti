# Example Prompts for Claude + Smriti

Once you've connected Smriti to Claude Desktop, use these prompts to explore its capabilities. Each prompt shows what Claude will do with the available Smriti tools.

---

## Knowledge Management: Creating and Organizing Notes

### 1. "Set up my research project workspace"
**What Claude does:**
- Creates multiple notes: project overview, research questions, key papers, findings log
- Adds metadata tags like `#research`, `#biology`, `#active`
- Links notes together to create a knowledge graph
- Stores project settings in agent memory (namespace: `projects:biology_research`)

### 2. "Create a note summarizing the key points from that article I just shared, and link it to my AI learning notes"
**What Claude does:**
- Uses `notes_create` to save the summary with tags like `#ai`, `#learning`, `#summary`
- Uses `notes_search` to find your existing AI learning notes
- Adds cross-references between the new note and related existing notes
- Uses `notes_graph` to visualize how this fits into your learning path

### 3. "Organize my meeting notes from today into a structured format with action items"
**What Claude does:**
- Creates a main meeting note with attendees, date, and discussion points
- Creates separate notes for each action item with owner and deadline
- Tags notes with `#meeting`, `#action-item`, participant names
- Links action items back to the main meeting note

### 4. "Search my notes for everything related to 'machine learning' and create a synthesis document"
**What Claude does:**
- Uses `notes_search` to find all notes tagged or containing "machine learning"
- Reviews the search results and identifies common themes
- Creates a new synthesis note that summarizes findings
- Uses `notes_graph` to show how different machine learning topics are connected

### 5. "What notes do I have about Python? Show me any connections to data science topics"
**What Claude does:**
- Uses `notes_list` to show all Python-related notes
- Uses `notes_graph` to find paths connecting Python notes to data science notes
- Describes the relationship between topics (e.g., "Python is used for data science in pandas and scikit-learn")
- Suggests new notes to create to fill gaps in the graph

---

## Agent Memory: Storing Preferences and Context

### 6. "Remember my note-taking preferences: I like markdown format, include timestamps, and organize by YYYY-MM-DD"
**What Claude does:**
- Uses `memory_store` to save preferences in namespace `user_preferences`
- Stores format preference, timestamp requirement, and date organization scheme
- Sets a long TTL (30 days or more) so preferences persist
- Confirms stored preferences and applies them to all future notes

### 7. "Store information about my current healthcare project in memory so you remember it across sessions"
**What Claude does:**
- Uses `memory_store` with namespace `healthcare:current_project`
- Stores project name, patient cohort info, key milestones, and contacts
- Sets appropriate TTL (e.g., until project completion)
- Confirms storage and retrieves it immediately to verify
- References this memory automatically in future conversations about healthcare

### 8. "What do you remember about my learning goals and study preferences?"
**What Claude does:**
- Uses `memory_retrieve` to look up stored learning goals and preferences
- Uses `memory_list` to show all memories in the `learning` namespace
- Summarizes what's stored and when it expires
- Suggests updating or adding new preferences if relevant

### 9. "Update my team's working hours in memory: we're in UTC timezone, working 9am-6pm"
**What Claude does:**
- Uses `memory_store` to save team working hours in namespace `team:working_hours`
- Stores timezone and hours
- Sets permanent storage (no TTL) since this is ongoing
- Uses this information automatically when scheduling or timing future work

### 10. "Clear expired memories and show me what current preferences and context I have stored"
**What Claude does:**
- Uses `memory_list` to retrieve all memories
- Identifies which memories have expired or are about to expire
- Summarizes current preferences, project contexts, and team information
- Suggests archiving or removing outdated memories

---

## Graph Discovery: Finding Connections and Insights

### 11. "Explore the graph of my research notes and find surprising connections I might have missed"
**What Claude does:**
- Uses `notes_graph` to map relationships between all notes
- Looks for notes that aren't directly connected but might be related
- Identifies clusters of related topics (e.g., methodology, biological systems, applications)
- Suggests new links or notes that would strengthen the knowledge graph
- Highlights unexpected connections that might spark new research directions

### 12. "Show me the shortest path between my notes on 'neural networks' and 'evolutionary biology'"
**What Claude does:**
- Uses `notes_graph` to find the connection path
- Returns intermediate notes that link the two topics
- Explains how each connection makes sense (shared concepts, authors, research areas)
- Suggests creating a new synthesis note if the path is convoluted

### 13. "What are my most connected notes? What topics appear to be central to my knowledge base?"
**What Claude does:**
- Uses `notes_list` to retrieve all notes with relationship metadata
- Uses `notes_graph` to calculate node centrality (how many other notes link to each)
- Identifies hub topics (highly connected) versus peripheral topics
- Suggests these as potential areas for deeper exploration or summary documents

---

## Healthcare Workflows: Care Coordination and Trial Tracking

### 14. "Create a patient care coordination note with linked information about medications, appointments, and providers"
**What Claude does:**
- Creates a main care coordination note with patient demographics and overview
- Creates linked notes for each medication (name, dosage, side effects, start date)
- Creates linked notes for each healthcare provider (name, specialty, contact)
- Creates a linked appointments tracker
- Stores patient ID and care team in agent memory with healthcare namespace
- Tags everything with `#healthcare`, `#care-coordination`, patient ID

### 15. "Set up tracking for a clinical trial participant including enrollment criteria, assessment schedule, and outcomes"
**What Claude does:**
- Creates trial protocol note with enrollment criteria and inclusion/exclusion rules
- Creates assessment schedule note with dates and required measurements
- Creates outcomes tracking note with primary and secondary endpoints
- Stores trial ID, participant cohort, and next assessment date in agent memory
- Uses `notes_graph` to link protocol to assessments to outcomes
- Sets memory TTL to match trial duration so context persists accurately
- Tags notes with `#clinical-trial`, trial ID, participant ID

---

## Advanced Workflows: Combining Multiple Tools

### 16. "I'm starting a new project. Set up a complete knowledge system: workspace notes, memory for preferences, and a graph structure"
**What Claude does:**
- Creates project folder with: overview, goals, methodology, timeline, resources notes
- Uses `memory_store` to save: project name, team members, deadline, budget, namespace reference
- Uses `notes_graph` to connect all initial notes
- Sets up tagging system and naming conventions
- Creates a "project hub" note that serves as central entry point
- Stores preferences for this project (format, update frequency, review schedule)

### 17. "I want to study a complex topic (e.g., climate change). Build me a learning path with organized notes and memory of what I've learned"
**What Claude does:**
- Creates a learning roadmap note (foundational concepts, intermediate topics, advanced areas)
- Creates individual notes for each concept with explanations and sources
- Uses `notes_graph` to structure concepts from basic to advanced
- Stores in agent memory: current progress, concepts mastered, next topic to study, learning pace preference
- Updates memory as you progress through topics
- Uses tags to show which concepts are "review", "learning", "mastered"

---

## Tips for Best Results

1. **Be descriptive:** Tell Claude about your context and goals. "Create a note about today's team meeting with action items" is better than "Create a note."

2. **Use namespaces:** When storing memories, specify a namespace to organize by project, domain, or person. Examples: `projects:alpha`, `healthcare:patient_123`, `user_preferences`

3. **Set TTLs wisely:** Use longer TTLs for stable information (preferences, team structure) and shorter TTLs for temporary context (current sprint, temporary focus area).

4. **Review your graph:** Periodically ask Claude to explore your knowledge graph to discover how your notes connect and identify gaps.

5. **Link intentionally:** When creating notes, ask Claude to link them to related existing notes. This builds a richer, more discoverable knowledge base.

6. **Search before creating:** Have Claude search existing notes before creating new ones to avoid duplicates and strengthen the graph.

7. **Use memory for context:** Store key preferences and project context in memory so Claude automatically remembers them without you repeating information each session.

---

## Example Multi-Tool Session

Here's what a complete workflow might look like:

> **You:** "I'm researching sustainable agriculture. Can you set up a knowledge workspace, store my learning preferences, and show me how to use the graph?"

> **Claude:**
> 1. Creates notes for: sustainability concepts, agricultural techniques, case studies, research papers
> 2. Uses `memory_store` to save: research focus, learning style preference, key sources, timeline
> 3. Uses `notes_graph` to connect concepts and show relationships
> 4. Asks clarifying questions about your goals and creates additional notes as needed

> **You:** "I found this new paper on crop rotation. Where does it fit in my knowledge base?"

> **Claude:**
> 1. Creates a note summarizing the paper
> 2. Uses `notes_graph` to find related topics (agricultural techniques, sustainability, soil health)
> 3. Links the new note to existing notes
> 4. Shows you where this paper fits in your research and suggests related areas to explore

This is the power of Claude + Smriti: persistent, interconnected knowledge with agent memory that understands your context and preferences.
