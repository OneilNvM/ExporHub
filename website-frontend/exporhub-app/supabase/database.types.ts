export type Json =
  | string
  | number
  | boolean
  | null
  | { [key: string]: Json | undefined }
  | Json[]

export type Database = {
  graphql_public: {
    Tables: {
      [_ in never]: never
    }
    Views: {
      [_ in never]: never
    }
    Functions: {
      graphql: {
        Args: {
          operationName?: string
          query?: string
          variables?: Json
          extensions?: Json
        }
        Returns: Json
      }
    }
    Enums: {
      [_ in never]: never
    }
    CompositeTypes: {
      [_ in never]: never
    }
  }
  public: {
    Tables: {
      __diesel_schema_migrations: {
        Row: {
          run_on: string
          version: string
        }
        Insert: {
          run_on?: string
          version: string
        }
        Update: {
          run_on?: string
          version?: string
        }
        Relationships: []
      }
      comment_dislikes: {
        Row: {
          comment_id: number
          dislike_id: number
          id: number
        }
        Insert: {
          comment_id: number
          dislike_id: number
          id?: number
        }
        Update: {
          comment_id?: number
          dislike_id?: number
          id?: number
        }
        Relationships: [
          {
            foreignKeyName: "comment_dislikes_comment_id_fkey"
            columns: ["comment_id"]
            isOneToOne: false
            referencedRelation: "comments"
            referencedColumns: ["comment_id"]
          },
          {
            foreignKeyName: "comment_dislikes_dislike_id_fkey"
            columns: ["dislike_id"]
            isOneToOne: false
            referencedRelation: "dislikes"
            referencedColumns: ["dislike_id"]
          },
        ]
      }
      comment_likes: {
        Row: {
          comment_id: number
          id: number
          like_id: number
        }
        Insert: {
          comment_id: number
          id?: number
          like_id: number
        }
        Update: {
          comment_id?: number
          id?: number
          like_id?: number
        }
        Relationships: [
          {
            foreignKeyName: "comment_likes_comment_id_fkey"
            columns: ["comment_id"]
            isOneToOne: false
            referencedRelation: "comments"
            referencedColumns: ["comment_id"]
          },
          {
            foreignKeyName: "comment_likes_like_id_fkey"
            columns: ["like_id"]
            isOneToOne: false
            referencedRelation: "likes"
            referencedColumns: ["like_id"]
          },
        ]
      }
      comments: {
        Row: {
          comment_id: number
          date: string
          project_id: number
          replies: number
          text: string
          user_id: number
        }
        Insert: {
          comment_id?: number
          date?: string
          project_id: number
          replies?: number
          text: string
          user_id: number
        }
        Update: {
          comment_id?: number
          date?: string
          project_id?: number
          replies?: number
          text?: string
          user_id?: number
        }
        Relationships: [
          {
            foreignKeyName: "comments_project_id_fkey"
            columns: ["project_id"]
            isOneToOne: false
            referencedRelation: "projects"
            referencedColumns: ["project_id"]
          },
          {
            foreignKeyName: "comments_user_id_fkey"
            columns: ["user_id"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["user_id"]
          },
        ]
      }
      dislikes: {
        Row: {
          date_disliked: string
          dislike_id: number
          user_id: number
        }
        Insert: {
          date_disliked?: string
          dislike_id?: number
          user_id: number
        }
        Update: {
          date_disliked?: string
          dislike_id?: number
          user_id?: number
        }
        Relationships: [
          {
            foreignKeyName: "dislikes_user_id_fkey"
            columns: ["user_id"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["user_id"]
          },
        ]
      }
      favourites: {
        Row: {
          date_favourited: string
          favourite_id: number
          project_id: number
          user_id: number
        }
        Insert: {
          date_favourited?: string
          favourite_id?: number
          project_id: number
          user_id: number
        }
        Update: {
          date_favourited?: string
          favourite_id?: number
          project_id?: number
          user_id?: number
        }
        Relationships: [
          {
            foreignKeyName: "favourites_project_id_fkey"
            columns: ["project_id"]
            isOneToOne: false
            referencedRelation: "projects"
            referencedColumns: ["project_id"]
          },
          {
            foreignKeyName: "favourites_user_id_fkey"
            columns: ["user_id"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["user_id"]
          },
        ]
      }
      follows: {
        Row: {
          date_followed: string
          follow_id: number
          follower: number
          following: number
        }
        Insert: {
          date_followed?: string
          follow_id?: number
          follower: number
          following: number
        }
        Update: {
          date_followed?: string
          follow_id?: number
          follower?: number
          following?: number
        }
        Relationships: [
          {
            foreignKeyName: "follows_follower_fkey"
            columns: ["follower"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["user_id"]
          },
          {
            foreignKeyName: "follows_following_fkey"
            columns: ["following"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["user_id"]
          },
        ]
      }
      images: {
        Row: {
          date_uploaded: string
          file_path: string
          image_id: number
          project_id: number | null
          user_id: number | null
        }
        Insert: {
          date_uploaded?: string
          file_path: string
          image_id?: number
          project_id?: number | null
          user_id?: number | null
        }
        Update: {
          date_uploaded?: string
          file_path?: string
          image_id?: number
          project_id?: number | null
          user_id?: number | null
        }
        Relationships: [
          {
            foreignKeyName: "images_project_id_fkey"
            columns: ["project_id"]
            isOneToOne: false
            referencedRelation: "projects"
            referencedColumns: ["project_id"]
          },
          {
            foreignKeyName: "images_user_id_fkey"
            columns: ["user_id"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["user_id"]
          },
        ]
      }
      likes: {
        Row: {
          date_liked: string
          like_id: number
          user_id: number
        }
        Insert: {
          date_liked?: string
          like_id?: number
          user_id: number
        }
        Update: {
          date_liked?: string
          like_id?: number
          user_id?: number
        }
        Relationships: [
          {
            foreignKeyName: "likes_user_id_fkey"
            columns: ["user_id"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["user_id"]
          },
        ]
      }
      projects: {
        Row: {
          date_created: string
          date_updated: string | null
          description: string
          favourites: number
          name: string
          project_id: number
          user_id: number
        }
        Insert: {
          date_created?: string
          date_updated?: string | null
          description: string
          favourites?: number
          name: string
          project_id?: number
          user_id: number
        }
        Update: {
          date_created?: string
          date_updated?: string | null
          description?: string
          favourites?: number
          name?: string
          project_id?: number
          user_id?: number
        }
        Relationships: [
          {
            foreignKeyName: "projects_user_id_fkey"
            columns: ["user_id"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["user_id"]
          },
        ]
      }
      replies: {
        Row: {
          date: string
          reply_id: number
          text: string
          user_id: number
        }
        Insert: {
          date?: string
          reply_id?: number
          text: string
          user_id: number
        }
        Update: {
          date?: string
          reply_id?: number
          text?: string
          user_id?: number
        }
        Relationships: [
          {
            foreignKeyName: "replies_user_id_fkey"
            columns: ["user_id"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["user_id"]
          },
        ]
      }
      reply_dislikes: {
        Row: {
          dislike_id: number
          id: number
          reply_id: number
        }
        Insert: {
          dislike_id: number
          id?: number
          reply_id: number
        }
        Update: {
          dislike_id?: number
          id?: number
          reply_id?: number
        }
        Relationships: [
          {
            foreignKeyName: "reply_dislikes_dislike_id_fkey"
            columns: ["dislike_id"]
            isOneToOne: false
            referencedRelation: "dislikes"
            referencedColumns: ["dislike_id"]
          },
          {
            foreignKeyName: "reply_dislikes_reply_id_fkey"
            columns: ["reply_id"]
            isOneToOne: false
            referencedRelation: "replies"
            referencedColumns: ["reply_id"]
          },
        ]
      }
      reply_likes: {
        Row: {
          id: number
          like_id: number
          reply_id: number
        }
        Insert: {
          id?: number
          like_id: number
          reply_id: number
        }
        Update: {
          id?: number
          like_id?: number
          reply_id?: number
        }
        Relationships: [
          {
            foreignKeyName: "reply_likes_like_id_fkey"
            columns: ["like_id"]
            isOneToOne: false
            referencedRelation: "likes"
            referencedColumns: ["like_id"]
          },
          {
            foreignKeyName: "reply_likes_reply_id_fkey"
            columns: ["reply_id"]
            isOneToOne: false
            referencedRelation: "replies"
            referencedColumns: ["reply_id"]
          },
        ]
      }
      threads: {
        Row: {
          comment_id: number
          reply_id: number
          thread_id: number
        }
        Insert: {
          comment_id: number
          reply_id: number
          thread_id?: number
        }
        Update: {
          comment_id?: number
          reply_id?: number
          thread_id?: number
        }
        Relationships: [
          {
            foreignKeyName: "threads_comment_id_fkey"
            columns: ["comment_id"]
            isOneToOne: false
            referencedRelation: "comments"
            referencedColumns: ["comment_id"]
          },
          {
            foreignKeyName: "threads_reply_id_fkey"
            columns: ["reply_id"]
            isOneToOne: false
            referencedRelation: "replies"
            referencedColumns: ["reply_id"]
          },
        ]
      }
      users: {
        Row: {
          bio: string | null
          date_created: string
          email: string
          followers: number
          password: string
          profile_img: string | null
          user_id: number
          username: string
        }
        Insert: {
          bio?: string | null
          date_created?: string
          email: string
          followers?: number
          password: string
          profile_img?: string | null
          user_id?: number
          username: string
        }
        Update: {
          bio?: string | null
          date_created?: string
          email?: string
          followers?: number
          password?: string
          profile_img?: string | null
          user_id?: number
          username?: string
        }
        Relationships: []
      }
    }
    Views: {
      [_ in never]: never
    }
    Functions: {
      diesel_manage_updated_at: {
        Args: { _tbl: unknown }
        Returns: undefined
      }
    }
    Enums: {
      [_ in never]: never
    }
    CompositeTypes: {
      [_ in never]: never
    }
  }
}

type DefaultSchema = Database[Extract<keyof Database, "public">]

export type Tables<
  DefaultSchemaTableNameOrOptions extends
    | keyof (DefaultSchema["Tables"] & DefaultSchema["Views"])
    | { schema: keyof Database },
  TableName extends DefaultSchemaTableNameOrOptions extends {
    schema: keyof Database
  }
    ? keyof (Database[DefaultSchemaTableNameOrOptions["schema"]]["Tables"] &
        Database[DefaultSchemaTableNameOrOptions["schema"]]["Views"])
    : never = never,
> = DefaultSchemaTableNameOrOptions extends { schema: keyof Database }
  ? (Database[DefaultSchemaTableNameOrOptions["schema"]]["Tables"] &
      Database[DefaultSchemaTableNameOrOptions["schema"]]["Views"])[TableName] extends {
      Row: infer R
    }
    ? R
    : never
  : DefaultSchemaTableNameOrOptions extends keyof (DefaultSchema["Tables"] &
        DefaultSchema["Views"])
    ? (DefaultSchema["Tables"] &
        DefaultSchema["Views"])[DefaultSchemaTableNameOrOptions] extends {
        Row: infer R
      }
      ? R
      : never
    : never

export type TablesInsert<
  DefaultSchemaTableNameOrOptions extends
    | keyof DefaultSchema["Tables"]
    | { schema: keyof Database },
  TableName extends DefaultSchemaTableNameOrOptions extends {
    schema: keyof Database
  }
    ? keyof Database[DefaultSchemaTableNameOrOptions["schema"]]["Tables"]
    : never = never,
> = DefaultSchemaTableNameOrOptions extends { schema: keyof Database }
  ? Database[DefaultSchemaTableNameOrOptions["schema"]]["Tables"][TableName] extends {
      Insert: infer I
    }
    ? I
    : never
  : DefaultSchemaTableNameOrOptions extends keyof DefaultSchema["Tables"]
    ? DefaultSchema["Tables"][DefaultSchemaTableNameOrOptions] extends {
        Insert: infer I
      }
      ? I
      : never
    : never

export type TablesUpdate<
  DefaultSchemaTableNameOrOptions extends
    | keyof DefaultSchema["Tables"]
    | { schema: keyof Database },
  TableName extends DefaultSchemaTableNameOrOptions extends {
    schema: keyof Database
  }
    ? keyof Database[DefaultSchemaTableNameOrOptions["schema"]]["Tables"]
    : never = never,
> = DefaultSchemaTableNameOrOptions extends { schema: keyof Database }
  ? Database[DefaultSchemaTableNameOrOptions["schema"]]["Tables"][TableName] extends {
      Update: infer U
    }
    ? U
    : never
  : DefaultSchemaTableNameOrOptions extends keyof DefaultSchema["Tables"]
    ? DefaultSchema["Tables"][DefaultSchemaTableNameOrOptions] extends {
        Update: infer U
      }
      ? U
      : never
    : never

export type Enums<
  DefaultSchemaEnumNameOrOptions extends
    | keyof DefaultSchema["Enums"]
    | { schema: keyof Database },
  EnumName extends DefaultSchemaEnumNameOrOptions extends {
    schema: keyof Database
  }
    ? keyof Database[DefaultSchemaEnumNameOrOptions["schema"]]["Enums"]
    : never = never,
> = DefaultSchemaEnumNameOrOptions extends { schema: keyof Database }
  ? Database[DefaultSchemaEnumNameOrOptions["schema"]]["Enums"][EnumName]
  : DefaultSchemaEnumNameOrOptions extends keyof DefaultSchema["Enums"]
    ? DefaultSchema["Enums"][DefaultSchemaEnumNameOrOptions]
    : never

export type CompositeTypes<
  PublicCompositeTypeNameOrOptions extends
    | keyof DefaultSchema["CompositeTypes"]
    | { schema: keyof Database },
  CompositeTypeName extends PublicCompositeTypeNameOrOptions extends {
    schema: keyof Database
  }
    ? keyof Database[PublicCompositeTypeNameOrOptions["schema"]]["CompositeTypes"]
    : never = never,
> = PublicCompositeTypeNameOrOptions extends { schema: keyof Database }
  ? Database[PublicCompositeTypeNameOrOptions["schema"]]["CompositeTypes"][CompositeTypeName]
  : PublicCompositeTypeNameOrOptions extends keyof DefaultSchema["CompositeTypes"]
    ? DefaultSchema["CompositeTypes"][PublicCompositeTypeNameOrOptions]
    : never

export const Constants = {
  graphql_public: {
    Enums: {},
  },
  public: {
    Enums: {},
  },
} as const
