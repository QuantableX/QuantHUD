export interface Note {
  id: string;
  title: string;
  content: string;
}

export interface Section {
  id: string;
  name: string;
  collapsed: boolean;
  notes: Note[];
}

const NOTES_KEY = "quanthud_notes";

function generateId(): string {
  return Date.now().toString(36) + Math.random().toString(36).slice(2, 8);
}

export function useNotes() {
  const sections = ref<Section[]>([]);

  // --- Persistence ---

  async function loadNotes() {
    try {
      if (typeof window !== "undefined" && (window as any).__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/core");
        const saved = await invoke<string>("load_config");
        if (saved) {
          const config = JSON.parse(saved);
          if (config._notes) {
            sections.value = config._notes;
            return;
          }
        }
      }
      const saved = localStorage.getItem(NOTES_KEY);
      if (saved) sections.value = JSON.parse(saved);
    } catch (e) {
      console.warn("Failed to load notes:", e);
    }
  }

  async function saveNotes() {
    try {
      const data = JSON.stringify(sections.value);
      if (typeof window !== "undefined" && (window as any).__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/core");
        const raw = await invoke<string>("load_config");
        const config = raw ? JSON.parse(raw) : {};
        config._notes = sections.value;
        await invoke("save_config", { config: JSON.stringify(config) });
      } else {
        localStorage.setItem(NOTES_KEY, data);
      }
    } catch (e) {
      console.warn("Failed to save notes:", e);
    }
  }

  // --- Section CRUD ---

  function addSection(name = "New Section") {
    sections.value.push({
      id: generateId(),
      name,
      collapsed: false,
      notes: [],
    });
    saveNotes();
  }

  function renameSection(sectionId: string, name: string) {
    const s = sections.value.find((s) => s.id === sectionId);
    if (s) {
      s.name = name;
      saveNotes();
    }
  }

  function deleteSection(sectionId: string) {
    sections.value = sections.value.filter((s) => s.id !== sectionId);
    saveNotes();
  }

  function toggleSection(sectionId: string) {
    const s = sections.value.find((s) => s.id === sectionId);
    if (s) {
      s.collapsed = !s.collapsed;
      saveNotes();
    }
  }

  function moveSectionUp(sectionId: string) {
    const idx = sections.value.findIndex((s) => s.id === sectionId);
    if (idx > 0) {
      const tmp = sections.value[idx];
      sections.value[idx] = sections.value[idx - 1];
      sections.value[idx - 1] = tmp;
      saveNotes();
    }
  }

  function moveSectionDown(sectionId: string) {
    const idx = sections.value.findIndex((s) => s.id === sectionId);
    if (idx < sections.value.length - 1) {
      const tmp = sections.value[idx];
      sections.value[idx] = sections.value[idx + 1];
      sections.value[idx + 1] = tmp;
      saveNotes();
    }
  }

  // --- Note CRUD ---

  function addNote(sectionId: string, title = "New Note") {
    const s = sections.value.find((s) => s.id === sectionId);
    if (s) {
      s.notes.push({ id: generateId(), title, content: "" });
      saveNotes();
    }
  }

  function renameNote(sectionId: string, noteId: string, title: string) {
    const s = sections.value.find((s) => s.id === sectionId);
    const n = s?.notes.find((n) => n.id === noteId);
    if (n) {
      n.title = title;
      saveNotes();
    }
  }

  function updateNoteContent(
    sectionId: string,
    noteId: string,
    content: string,
  ) {
    const s = sections.value.find((s) => s.id === sectionId);
    const n = s?.notes.find((n) => n.id === noteId);
    if (n) {
      n.content = content;
      saveNotes();
    }
  }

  function deleteNote(sectionId: string, noteId: string) {
    const s = sections.value.find((s) => s.id === sectionId);
    if (s) {
      s.notes = s.notes.filter((n) => n.id !== noteId);
      saveNotes();
    }
  }

  function moveNoteUp(sectionId: string, noteId: string) {
    const s = sections.value.find((s) => s.id === sectionId);
    if (!s) return;
    const idx = s.notes.findIndex((n) => n.id === noteId);
    if (idx > 0) {
      const tmp = s.notes[idx];
      s.notes[idx] = s.notes[idx - 1];
      s.notes[idx - 1] = tmp;
      saveNotes();
    }
  }

  function moveNoteDown(sectionId: string, noteId: string) {
    const s = sections.value.find((s) => s.id === sectionId);
    if (!s) return;
    const idx = s.notes.findIndex((n) => n.id === noteId);
    if (idx < s.notes.length - 1) {
      const tmp = s.notes[idx];
      s.notes[idx] = s.notes[idx + 1];
      s.notes[idx + 1] = tmp;
      saveNotes();
    }
  }

  function moveNoteToSection(
    fromSectionId: string,
    noteId: string,
    toSectionId: string,
  ) {
    const from = sections.value.find((s) => s.id === fromSectionId);
    const to = sections.value.find((s) => s.id === toSectionId);
    if (!from || !to) return;
    const noteIdx = from.notes.findIndex((n) => n.id === noteId);
    if (noteIdx === -1) return;
    const [note] = from.notes.splice(noteIdx, 1);
    to.notes.push(note);
    saveNotes();
  }

  // --- Duplicate helpers ---

  function duplicateSection(sectionId: string) {
    const s = sections.value.find((s) => s.id === sectionId);
    if (!s) return;
    const idx = sections.value.indexOf(s);
    const newSection: Section = {
      id: generateId(),
      name: s.name + " (copy)",
      collapsed: false,
      notes: s.notes.map((n) => ({
        id: generateId(),
        title: n.title,
        content: n.content,
      })),
    };
    sections.value.splice(idx + 1, 0, newSection);
    saveNotes();
  }

  function duplicateNote(sectionId: string, noteId: string) {
    const s = sections.value.find((s) => s.id === sectionId);
    if (!s) return;
    const nIdx = s.notes.findIndex((n) => n.id === noteId);
    if (nIdx === -1) return;
    const original = s.notes[nIdx];
    const newNote: Note = {
      id: generateId(),
      title: original.title + " (copy)",
      content: original.content,
    };
    s.notes.splice(nIdx + 1, 0, newNote);
    saveNotes();
  }

  // --- Drag & drop reorder helpers ---

  function reorderSection(fromIndex: number, toIndex: number) {
    if (fromIndex === toIndex) return;
    const [moved] = sections.value.splice(fromIndex, 1);
    sections.value.splice(toIndex, 0, moved);
    saveNotes();
  }

  function reorderNote(sectionId: string, fromIndex: number, toIndex: number) {
    const s = sections.value.find((s) => s.id === sectionId);
    if (!s || fromIndex === toIndex) return;
    const [moved] = s.notes.splice(fromIndex, 1);
    s.notes.splice(toIndex, 0, moved);
    saveNotes();
  }

  function moveNoteToSectionAt(
    fromSectionId: string,
    noteIndex: number,
    toSectionId: string,
    toIndex: number,
  ) {
    const from = sections.value.find((s) => s.id === fromSectionId);
    const to = sections.value.find((s) => s.id === toSectionId);
    if (!from || !to) return;
    const [note] = from.notes.splice(noteIndex, 1);
    to.notes.splice(toIndex, 0, note);
    saveNotes();
  }

  onMounted(() => {
    loadNotes();
  });

  return {
    sections,
    addSection,
    renameSection,
    deleteSection,
    toggleSection,
    moveSectionUp,
    moveSectionDown,
    addNote,
    renameNote,
    updateNoteContent,
    deleteNote,
    moveNoteUp,
    moveNoteDown,
    moveNoteToSection,
    reorderSection,
    reorderNote,
    moveNoteToSectionAt,
    duplicateSection,
    duplicateNote,
  };
}
