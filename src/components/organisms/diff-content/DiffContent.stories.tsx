import type { Meta, StoryObj } from '@storybook/react'

import { DiffContent } from './DiffContent'

const meta = {
  title: 'Organisms/DiffContent',
  component: DiffContent,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof DiffContent>

export default meta
type Story = StoryObj<typeof meta>

export const RowDeleted: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'deleted', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
      },
      rowDiffs2: {},
    },
  },
}

export const RowAdded: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {},
      rowDiffs2: {
        '1': {
          name: { status: 'added', value: '"John"' },
          age: { status: 'added', value: '29' },
        },
      },
    },
  },
}

export const RowModified: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'deleted', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
      },
      rowDiffs2: {
        '1': {
          name: { status: 'added', value: '"Jane"' },
          age: { status: 'added', value: '15' },
        },
      },
    },
  },
}

export const RowsDeleted: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1', '2'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'deleted', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
        '2': {
          name: { status: 'deleted', value: '"Alice"' },
          age: { status: 'deleted', value: '31' },
        },
      },
      rowDiffs2: {},
    },
  },
}

export const RowsAdded: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1', '2'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {},
      rowDiffs2: {
        '1': {
          name: { status: 'added', value: '"John"' },
          age: { status: 'added', value: '29' },
        },
        '2': {
          name: { status: 'added', value: '"Alice"' },
          age: { status: 'added', value: '31' },
        },
      },
    },
  },
}

export const RowsModified: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1', '2'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'deleted', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
        '2': {
          name: { status: 'deleted', value: '"Alice"' },
          age: { status: 'deleted', value: '31' },
        },
      },
      rowDiffs2: {
        '1': {
          name: { status: 'added', value: '"Jane"' },
          age: { status: 'added', value: '15' },
        },
        '2': {
          name: { status: 'added', value: '"Bob"' },
          age: { status: 'added', value: '42' },
        },
      },
    },
  },
}

export const RowsModifiedAndDeleted: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1', '2'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'deleted', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
        '2': {
          name: { status: 'deleted', value: '"Alice"' },
          age: { status: 'deleted', value: '31' },
        },
      },
      rowDiffs2: {
        '1': {
          name: { status: 'added', value: '"Jane"' },
          age: { status: 'added', value: '15' },
        },
      },
    },
  },
}

export const RowsModifiedAndAdded: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1', '2'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'deleted', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
      },
      rowDiffs2: {
        '1': {
          name: { status: 'added', value: '"Jane"' },
          age: { status: 'added', value: '15' },
        },
        '2': {
          name: { status: 'added', value: '"Alice"' },
          age: { status: 'added', value: '31' },
        },
      },
    },
  },
}

export const ColModified: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'stay', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
      },
      rowDiffs2: {
        '1': {
          name: { status: 'stay', value: '"John"' },
          age: { status: 'added', value: '15' },
        },
      },
    },
  },
}

export const ColRemoved: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'stay', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
      },
      rowDiffs2: {
        '1': {
          name: { status: 'stay', value: '"John"' },
        },
      },
    },
  },
}

export const ColCreated: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'stay', value: '"John"' },
        },
      },
      rowDiffs2: {
        '1': {
          name: { status: 'stay', value: '"John"' },
          age: { status: 'added', value: '29' },
        },
      },
    },
  },
}

export const ColMismatched: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1'],
      primaryColName: 'id',
      colNames: ['name', 'age', 'rate'],
      rowDiffs1: {
        '1': {
          name: { status: 'stay', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
      },
      rowDiffs2: {
        '1': {
          name: { status: 'stay', value: '"John"' },
          rate: { status: 'added', value: '1' },
        },
      },
    },
  },
}

export const RowModifiedStringId: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['"EF974256-0BDE-4170-A2FC-4BDBBD696FB5"'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '"EF974256-0BDE-4170-A2FC-4BDBBD696FB5"': {
          name: { status: 'deleted', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
      },
      rowDiffs2: {
        '"EF974256-0BDE-4170-A2FC-4BDBBD696FB5"': {
          name: { status: 'added', value: '"Jane"' },
          age: { status: 'added', value: '15' },
        },
      },
    },
  },
}

export const RowsModifiedMultiUniqueColumn: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1-"001"', '1-"002"', '2-"001"'],
      primaryColName: 'id-code',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1-"001"': {
          name: { status: 'deleted', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
        '2-"001"': {
          name: { status: 'stay', value: '"Alice"' },
          age: { status: 'deleted', value: '25' },
        },
      },
      rowDiffs2: {
        '1-"002"': {
          name: { status: 'added', value: '"John"' },
          age: { status: 'added', value: '29' },
        },
        '2-"001"': {
          name: { status: 'stay', value: '"Alice"' },
          age: { status: 'added', value: '31' },
        },
      },
    },
  },
}

export const RowAddOnlyUniqueColumn: Story = {
  args: {
    tableDiff: {
      tableName: 'relations',
      primaryValues: ['1-"001"'],
      primaryColName: 'id-code',
      colNames: [],
      rowDiffs1: {
        '1-"001"': {},
      },
      rowDiffs2: {},
    },
  },
}

export const RowDeletedOnlyUniqueColumn: Story = {
  args: {
    tableDiff: {
      tableName: 'relations',
      primaryValues: ['1-"002"'],
      primaryColName: 'id-code',
      colNames: [],
      rowDiffs1: {},
      rowDiffs2: {
        '1-"002"': {},
      },
    },
  },
}

export const RowModifiedOnlyUniqueColumn: Story = {
  args: {
    tableDiff: {
      tableName: 'relations',
      primaryValues: ['1-"001"', '1-"002"'],
      primaryColName: 'id-code',
      colNames: [],
      rowDiffs1: {
        '1-"001"': {},
      },
      rowDiffs2: {
        '1-"002"': {},
      },
    },
  },
}

export const LargeDiff: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1', '23456'],
      primaryColName: 'id',
      colNames: ['name-value', 'age', 'session', 'note', 'created', 'updated'],
      rowDiffs1: {
        '1': {
          'name-value': { status: 'stay', value: '"John"' },
          age: { status: 'stay', value: '29' },
          session: {
            status: 'deleted',
            value:
              '"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.NHVaYe26MbtOYhSKkoKYdFVomg4i8ZJd8_-RU8VNbftc4TSMb4bXP3l3YlNWACwyXPGffz5aXHc6lty1Y2t4SWRqGteragsVdZufDn5BlnJl9pdR_kdVFUsra2rWKEofkZeIC4yWytE58sMIihvo9H1ScmmVwBcQP6XETqYd0aSHp1gOa9RdUPDvoXQ5oqygTqVtxaDr6wUFKrKItgBMzWIdNZ6y7O9E0DhEPTbE9rfBo6KTFsHAZnMg4k68CDp2woYIaXbmYTWcvbzIuHO7_37GT79XdIwkm95QJ7hYC9RiwrV7mesbY4PAahERJawntho0my942XheVLmGwLMBkQ"',
          },
          note: {
            status: 'deleted',
            value:
              '"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."',
          },
          created: { status: 'stay', value: '"2023-06-12 11:30:56"' },
          updated: { status: 'deleted', value: '"2023-06-15 08:42:05"' },
        },
      },
      rowDiffs2: {
        '1': {
          'name-value': { status: 'stay', value: '"Alexander"' },
          age: { status: 'stay', value: '29' },
          session: {
            status: 'added',
            value:
              '"eyJhbGciOiJSUzM4NCIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.o1hC1xYbJolSyh0-bOY230w22zEQSk5TiBfc-OCvtpI2JtYlW-23-8B48NpATozzMHn0j3rE0xVUldxShzy0xeJ7vYAccVXu2Gs9rnTVqouc-UZu_wJHkZiKBL67j8_61L6SXswzPAQu4kVDwAefGf5hyYBUM-80vYZwWPEpLI8K4yCBsF6I9N1yQaZAJmkMp_Iw371Menae4Mp4JusvBJS-s6LrmG2QbiZaFaxVJiW8KlUkWyUCns8-qFl5OMeYlgGFsyvvSHvXCzQrsEXqyCdS4tQJd73ayYA4SPtCb9clz76N1zE5WsV4Z0BYrxeb77oA7jJhh994RAPzCG0hmQ"',
          },
          note: {
            status: 'added',
            value:
              '"Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n"',
          },
          created: { status: 'stay', value: '"2023-06-12 11:30:56"' },
          updated: { status: 'added', value: '"2023-06-17 23:00:17"' },
        },
        '23456': {
          'name-value': { status: 'added', value: '"Smith"' },
          age: { status: 'added', value: '42' },
          session: {
            status: 'added',
            value: '<null>',
          },
          note: {
            status: 'added',
            value: '',
          },
          created: { status: 'added', value: '"2023-06-12 11:30:56"' },
          updated: { status: 'added', value: '"2023-06-17 23:00:17"' },
        },
      },
    },
  },
}
