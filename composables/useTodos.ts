export interface SubTask {
  id: string;
  title: string;
  done: boolean;
}

export interface Task {
  id: string;
  title: string;
  done: boolean;
  expanded: boolean;
  subtasks: SubTask[];
}

export interface TodoSection {
  id: string;
  name: string;
  collapsed: boolean;
  tasks: Task[];
}

const TODOS_KEY = "quanthud_todos";

function generateId(): string {
  return Date.now().toString(36) + Math.random().toString(36).slice(2, 8);
}

export function useTodos() {
  const sections = ref<TodoSection[]>([]);

  // --- Persistence ---

  async function loadTodos() {
    try {
      if (typeof window !== "undefined" && (window as any).__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/core");
        const saved = await invoke<string>("load_config");
        if (saved) {
          const config = JSON.parse(saved);
          if (config._todos) {
            sections.value = config._todos;
            migrateTasks();
            return;
          }
        }
      }
      const saved = localStorage.getItem(TODOS_KEY);
      if (saved) sections.value = JSON.parse(saved);
      migrateTasks();
    } catch (e) {
      console.warn("Failed to load todos:", e);
    }
  }

  function migrateTasks() {
    for (const section of sections.value) {
      for (const task of section.tasks) {
        if (task.expanded === undefined) task.expanded = false;
      }
    }
  }

  async function saveTodos() {
    try {
      const data = JSON.stringify(sections.value);
      if (typeof window !== "undefined" && (window as any).__TAURI__) {
        const { invoke } = await import("@tauri-apps/api/core");
        const raw = await invoke<string>("load_config");
        const config = raw ? JSON.parse(raw) : {};
        config._todos = sections.value;
        await invoke("save_config", { config: JSON.stringify(config) });
      } else {
        localStorage.setItem(TODOS_KEY, data);
      }
    } catch (e) {
      console.warn("Failed to save todos:", e);
    }
  }

  // --- Section CRUD ---

  function addSection(name = "New Section") {
    sections.value.push({
      id: generateId(),
      name,
      collapsed: false,
      tasks: [],
    });
    saveTodos();
  }

  function renameSection(sectionId: string, name: string) {
    const s = sections.value.find((s) => s.id === sectionId);
    if (s) {
      s.name = name;
      saveTodos();
    }
  }

  function deleteSection(sectionId: string) {
    sections.value = sections.value.filter((s) => s.id !== sectionId);
    saveTodos();
  }

  function toggleSection(sectionId: string) {
    const s = sections.value.find((s) => s.id === sectionId);
    if (s) {
      s.collapsed = !s.collapsed;
      saveTodos();
    }
  }

  // --- Task CRUD ---

  function addTask(sectionId: string, title = "New Task") {
    const s = sections.value.find((s) => s.id === sectionId);
    if (s) {
      s.tasks.push({
        id: generateId(),
        title,
        done: false,
        expanded: false,
        subtasks: [],
      });
      saveTodos();
    }
  }

  function renameTask(sectionId: string, taskId: string, title: string) {
    const t = sections.value
      .find((s) => s.id === sectionId)
      ?.tasks.find((t) => t.id === taskId);
    if (t) {
      t.title = title;
      saveTodos();
    }
  }

  function toggleTask(sectionId: string, taskId: string) {
    const t = sections.value
      .find((s) => s.id === sectionId)
      ?.tasks.find((t) => t.id === taskId);
    if (t) {
      t.done = !t.done;
      saveTodos();
    }
  }

  function deleteTask(sectionId: string, taskId: string) {
    const s = sections.value.find((s) => s.id === sectionId);
    if (s) {
      s.tasks = s.tasks.filter((t) => t.id !== taskId);
      saveTodos();
    }
  }

  // --- Subtask CRUD ---

  function addSubtask(
    sectionId: string,
    taskId: string,
    title = "New Subtask",
  ) {
    const t = sections.value
      .find((s) => s.id === sectionId)
      ?.tasks.find((t) => t.id === taskId);
    if (t) {
      t.subtasks.push({ id: generateId(), title, done: false });
      saveTodos();
    }
  }

  function renameSubtask(
    sectionId: string,
    taskId: string,
    subtaskId: string,
    title: string,
  ) {
    const st = sections.value
      .find((s) => s.id === sectionId)
      ?.tasks.find((t) => t.id === taskId)
      ?.subtasks.find((st) => st.id === subtaskId);
    if (st) {
      st.title = title;
      saveTodos();
    }
  }

  function toggleSubtask(sectionId: string, taskId: string, subtaskId: string) {
    const st = sections.value
      .find((s) => s.id === sectionId)
      ?.tasks.find((t) => t.id === taskId)
      ?.subtasks.find((st) => st.id === subtaskId);
    if (st) {
      st.done = !st.done;
      saveTodos();
    }
  }

  function deleteSubtask(sectionId: string, taskId: string, subtaskId: string) {
    const t = sections.value
      .find((s) => s.id === sectionId)
      ?.tasks.find((t) => t.id === taskId);
    if (t) {
      t.subtasks = t.subtasks.filter((st) => st.id !== subtaskId);
      saveTodos();
    }
  }

  // --- Duplicate helpers ---

  function duplicateSection(sectionId: string) {
    const s = sections.value.find((s) => s.id === sectionId);
    if (!s) return;
    const idx = sections.value.indexOf(s);
    const newSection: TodoSection = {
      id: generateId(),
      name: s.name + " (copy)",
      collapsed: false,
      tasks: s.tasks.map((t) => ({
        id: generateId(),
        title: t.title,
        done: t.done,
        expanded: t.expanded,
        subtasks: t.subtasks.map((st) => ({
          id: generateId(),
          title: st.title,
          done: st.done,
        })),
      })),
    };
    sections.value.splice(idx + 1, 0, newSection);
    saveTodos();
  }

  function duplicateTask(sectionId: string, taskId: string) {
    const s = sections.value.find((s) => s.id === sectionId);
    if (!s) return;
    const tIdx = s.tasks.findIndex((t) => t.id === taskId);
    if (tIdx === -1) return;
    const original = s.tasks[tIdx];
    const newTask: Task = {
      id: generateId(),
      title: original.title + " (copy)",
      done: original.done,
      expanded: original.expanded,
      subtasks: original.subtasks.map((st) => ({
        id: generateId(),
        title: st.title,
        done: st.done,
      })),
    };
    s.tasks.splice(tIdx + 1, 0, newTask);
    saveTodos();
  }

  // --- Reorder helpers ---

  function reorderSection(fromIndex: number, toIndex: number) {
    if (fromIndex === toIndex) return;
    const [moved] = sections.value.splice(fromIndex, 1);
    sections.value.splice(toIndex, 0, moved);
    saveTodos();
  }

  function reorderTask(sectionId: string, fromIndex: number, toIndex: number) {
    const s = sections.value.find((s) => s.id === sectionId);
    if (!s || fromIndex === toIndex) return;
    const [moved] = s.tasks.splice(fromIndex, 1);
    s.tasks.splice(toIndex, 0, moved);
    saveTodos();
  }

  function moveTaskToSectionAt(
    fromSectionId: string,
    taskIndex: number,
    toSectionId: string,
    toIndex: number,
  ) {
    const from = sections.value.find((s) => s.id === fromSectionId);
    const to = sections.value.find((s) => s.id === toSectionId);
    if (!from || !to) return;
    const [task] = from.tasks.splice(taskIndex, 1);
    to.tasks.splice(toIndex, 0, task);
    saveTodos();
  }

  function reorderSubtask(
    sectionId: string,
    taskId: string,
    fromIndex: number,
    toIndex: number,
  ) {
    const t = sections.value
      .find((s) => s.id === sectionId)
      ?.tasks.find((t) => t.id === taskId);
    if (!t || fromIndex === toIndex) return;
    const [moved] = t.subtasks.splice(fromIndex, 1);
    t.subtasks.splice(toIndex, 0, moved);
    saveTodos();
  }

  function moveSubtaskToTask(
    fromSectionId: string,
    fromTaskId: string,
    subtaskIndex: number,
    toSectionId: string,
    toTaskId: string,
    toIndex: number,
  ) {
    const fromTask = sections.value
      .find((s) => s.id === fromSectionId)
      ?.tasks.find((t) => t.id === fromTaskId);
    const toTask = sections.value
      .find((s) => s.id === toSectionId)
      ?.tasks.find((t) => t.id === toTaskId);
    if (!fromTask || !toTask) return;
    const [subtask] = fromTask.subtasks.splice(subtaskIndex, 1);
    toTask.subtasks.splice(toIndex, 0, subtask);
    saveTodos();
  }

  function toggleTaskExpand(sectionId: string, taskId: string) {
    const t = sections.value
      .find((s) => s.id === sectionId)
      ?.tasks.find((t) => t.id === taskId);
    if (t) {
      t.expanded = !t.expanded;
      saveTodos();
    }
  }

  onMounted(() => {
    loadTodos();
  });

  return {
    sections,
    addSection,
    renameSection,
    deleteSection,
    toggleSection,
    addTask,
    renameTask,
    toggleTask,
    deleteTask,
    addSubtask,
    renameSubtask,
    toggleSubtask,
    deleteSubtask,
    reorderSection,
    reorderTask,
    moveTaskToSectionAt,
    reorderSubtask,
    moveSubtaskToTask,
    toggleTaskExpand,
    duplicateSection,
    duplicateTask,
  };
}
